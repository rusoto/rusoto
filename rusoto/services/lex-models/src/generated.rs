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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Provides information about a bot alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BotAliasMetadata {
    /// <p>The name of the bot to which the alias points.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The version of the Amazon Lex bot to which the alias points.</p>
    #[serde(rename = "botVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    /// <p>Checksum of the bot alias.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The date that the bot alias was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot alias.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the bot alias was updated. When you create a resource, the creation date and last updated date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the bot alias.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents an association between an Amazon Lex bot and an external messaging platform.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BotChannelAssociation {
    /// <p>An alias pointing to the specific version of the Amazon Lex bot to which this association is being made. </p>
    #[serde(rename = "botAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias: Option<String>,
    /// <p>Provides information necessary to communicate with the messaging platform. </p>
    #[serde(rename = "botConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The name of the Amazon Lex bot to which this association is being made. </p> <note> <p>Currently, Amazon Lex supports associations with Facebook and Slack, and Twilio.</p> </note></p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The date that the association between the Amazon Lex bot and the channel was created. </p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A text description of the association you are creating. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the reason that it failed to create the association.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the association between the bot and the channel. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the bot channel. </p> <ul> <li> <p> <code>CREATED</code> - The channel has been created and is ready for use.</p> </li> <li> <p> <code>IN_PROGRESS</code> - Channel creation is in progress.</p> </li> <li> <p> <code>FAILED</code> - There was an error creating the channel. For information about the reason for the failure, see the <code>failureReason</code> field.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies the type of association by indicating the type of channel being established between the Amazon Lex bot and the external messaging platform.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information about a bot. .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BotMetadata {
    /// <p>The date that the bot was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the bot was updated. When you create a bot, the creation date and last updated date are the same. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the bot. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the bot.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version of the bot. For a new bot, the version is always <code>$LATEST</code>.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Provides metadata for a built-in intent.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuiltinIntentMetadata {
    /// <p>A unique identifier for the built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>A list of identifiers for the locales that the intent supports.</p>
    #[serde(rename = "supportedLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locales: Option<Vec<String>>,
}

/// <p>Provides information about a slot used in a built-in intent.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuiltinIntentSlot {
    /// <p>A list of the slots defined for the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Provides information about a built in slot type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BuiltinSlotTypeMetadata {
    /// <p>A unique identifier for the built-in slot type. To find the signature for a slot type, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>A list of target locales for the slot. </p>
    #[serde(rename = "supportedLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locales: Option<Vec<String>>,
}

/// <p>Specifies a Lambda function that verifies requests to a bot or fulfills the user's request to a bot..</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeHook {
    /// <p>The version of the request-response that you want Amazon Lex to use to invoke your Lambda function. For more information, see <a>using-lambda</a>.</p>
    #[serde(rename = "messageVersion")]
    pub message_version: String,
    /// <p>The Amazon Resource Name (ARN) of the Lambda function.</p>
    #[serde(rename = "uri")]
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBotVersionRequest {
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version of the bot. If you specify a checksum and the <code>$LATEST</code> version of the bot has a different checksum, a <code>PreconditionFailedException</code> exception is returned and Amazon Lex doesn't publish a new version. If you don't specify a checksum, Amazon Lex publishes the <code>$LATEST</code> version.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The name of the bot that you want to create a new version of. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateBotVersionResponse {
    /// <p>The message that Amazon Lex uses to abort a conversation. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "abortStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_statement: Option<Statement>,
    /// <p>Checksum identifying the version of the bot that was created.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service, you must specify whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to the Children's Online Privacy Protection Act (COPPA) by specifying <code>true</code> or <code>false</code> in the <code>childDirected</code> field. By specifying <code>true</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. By specifying <code>false</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is not</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. You may not specify a default value for the <code>childDirected</code> field that does not accurately reflect whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.</p> <p>If your use of Amazon Lex relates to a website, program, or other application that is directed in whole or in part, to children under age 13, you must obtain any required verifiable parental consent under COPPA. For information regarding the use of Amazon Lex in connection with websites, programs, or other applications that are directed or targeted, in whole or in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a> </p>
    #[serde(rename = "childDirected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_directed: Option<bool>,
    /// <p>The message that Amazon Lex uses when it doesn't understand the user's request. For more information, see <a>PutBot</a>. </p>
    #[serde(rename = "clarificationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_prompt: Option<Prompt>,
    /// <p>The date when the bot version was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the reason that it failed to build the bot.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The maximum time in seconds that Amazon Lex retains the data gathered in a conversation. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_ttl_in_seconds: Option<i64>,
    /// <p>An array of <code>Intent</code> objects. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    /// <p>The date when the <code>$LATEST</code> version of this bot was updated. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p> Specifies the target locale for the bot. </p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The name of the bot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> When you send a request to create or update a bot, Amazon Lex sets the <code>status</code> response element to <code>BUILDING</code>. After Amazon Lex builds the bot, it sets <code>status</code> to <code>READY</code>. If Amazon Lex can't build the bot, it sets <code>status</code> to <code>FAILED</code>. Amazon Lex returns the reason for the failure in the <code>failureReason</code> response element. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version of the bot. </p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The Amazon Polly voice ID that Amazon Lex uses for voice interactions with the user.</p>
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateIntentVersionRequest {
    /// <p>Checksum of the <code>$LATEST</code> version of the intent that should be used to create the new version. If you specify a checksum and the <code>$LATEST</code> version of the intent has a different checksum, Amazon Lex returns a <code>PreconditionFailedException</code> exception and doesn't publish a new version. If you don't specify a checksum, Amazon Lex publishes the <code>$LATEST</code> version.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The name of the intent that you want to create a new version of. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateIntentVersionResponse {
    /// <p>Checksum of the intent version created.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>After the Lambda function specified in the <code>fulfillmentActivity</code> field fulfills the intent, Amazon Lex conveys this statement to the user. </p>
    #[serde(rename = "conclusionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion_statement: Option<Statement>,
    /// <p>If defined, the prompt that Amazon Lex uses to confirm the user's intent before fulfilling it. </p>
    #[serde(rename = "confirmationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_prompt: Option<Prompt>,
    /// <p>The date that the intent was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the intent.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If defined, Amazon Lex invokes this Lambda function for each user input.</p>
    #[serde(rename = "dialogCodeHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<CodeHook>,
    /// <p>If defined, Amazon Lex uses this prompt to solicit additional user activity after the intent is fulfilled. </p>
    #[serde(rename = "followUpPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_prompt: Option<FollowUpPrompt>,
    /// <p> Describes how the intent is fulfilled. </p>
    #[serde(rename = "fulfillmentActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_activity: Option<FulfillmentActivity>,
    /// <p>The date that the intent was updated. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a built-in intent.</p>
    #[serde(rename = "parentIntentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    /// <p>If the user answers "no" to the question defined in <code>confirmationPrompt</code>, Amazon Lex responds with this statement to acknowledge that the intent was canceled. </p>
    #[serde(rename = "rejectionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_statement: Option<Statement>,
    /// <p>An array of sample utterances configured for the intent. </p>
    #[serde(rename = "sampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>An array of slot types that defines the information required to fulfill the intent.</p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Slot>>,
    /// <p>The version number assigned to the new version of the intent.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSlotTypeVersionRequest {
    /// <p>Checksum for the <code>$LATEST</code> version of the slot type that you want to publish. If you specify a checksum and the <code>$LATEST</code> version of the slot type has a different checksum, Amazon Lex returns a <code>PreconditionFailedException</code> exception and doesn't publish the new version. If you don't specify a checksum, Amazon Lex publishes the <code>$LATEST</code> version.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The name of the slot type that you want to create a new version for. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSlotTypeVersionResponse {
    /// <p>Checksum of the <code>$LATEST</code> version of the slot type.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The date that the slot type was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the slot type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of <code>EnumerationValue</code> objects that defines the values that the slot type can take.</p>
    #[serde(rename = "enumerationValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration_values: Option<Vec<EnumerationValue>>,
    /// <p>The date that the slot type was updated. When you create a resource, the creation date and last update date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The strategy that Amazon Lex uses to determine the value of the slot. For more information, see <a>PutSlotType</a>.</p>
    #[serde(rename = "valueSelectionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_strategy: Option<String>,
    /// <p>The version assigned to the new slot type version. </p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBotAliasRequest {
    /// <p>The name of the bot that the alias points to.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the alias to delete. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBotChannelAssociationRequest {
    /// <p>An alias that points to the specific version of the Amazon Lex bot to which this association is being made.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the Amazon Lex bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the association. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBotRequest {
    /// <p>The name of the bot. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBotVersionRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the bot to delete. You cannot delete the <code>$LATEST</code> version of the bot. To delete the <code>$LATEST</code> version, use the <a>DeleteBot</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntentRequest {
    /// <p>The name of the intent. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntentVersionRequest {
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the intent to delete. You cannot delete the <code>$LATEST</code> version of the intent. To delete the <code>$LATEST</code> version, use the <a>DeleteIntent</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSlotTypeRequest {
    /// <p>The name of the slot type. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSlotTypeVersionRequest {
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the slot type to delete. You cannot delete the <code>$LATEST</code> version of the slot type. To delete the <code>$LATEST</code> version, use the <a>DeleteSlotType</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUtterancesRequest {
    /// <p>The name of the bot that stored the utterances.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p> The unique identifier for the user that made the utterances. This is the user ID that was sent in the <a href="http://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostContent.html">PostContent</a> or <a href="http://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostText.html">PostText</a> operation request that contained the utterance.</p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

/// <p><p>Each slot type can have a set of values. Each enumeration value represents a value the slot type can take. </p> <p>For example, a pizza ordering bot could have a slot type that specifies the type of crust that the pizza should have. The slot type could include the values </p> <ul> <li> <p>thick</p> </li> <li> <p>thin</p> </li> <li> <p>stuffed</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumerationValue {
    /// <p>Additional values related to the slot type value.</p>
    #[serde(rename = "synonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    /// <p>The value of the slot type.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>A prompt for additional activity after an intent is fulfilled. For example, after the <code>OrderPizza</code> intent is fulfilled, you might prompt the user to find out whether the user wants to order drinks.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowUpPrompt {
    /// <p>Prompts for information from the user. </p>
    #[serde(rename = "prompt")]
    pub prompt: Prompt,
    /// <p>If the user answers "no" to the question defined in the <code>prompt</code> field, Amazon Lex responds with this statement to acknowledge that the intent was canceled. </p>
    #[serde(rename = "rejectionStatement")]
    pub rejection_statement: Statement,
}

/// <p><p> Describes how the intent is fulfilled after the user provides all of the information required for the intent. You can provide a Lambda function to process the intent, or you can return the intent information to the client application. We recommend that you use a Lambda function so that the relevant logic lives in the Cloud and limit the client-side code primarily to presentation. If you need to update the logic, you only update the Lambda function; you don&#39;t need to upgrade your client application. </p> <p>Consider the following examples:</p> <ul> <li> <p>In a pizza ordering application, after the user provides all of the information for placing an order, you use a Lambda function to place an order with a pizzeria. </p> </li> <li> <p>In a gaming application, when a user says &quot;pick up a rock,&quot; this information must go back to the client application so that it can perform the operation and update the graphics. In this case, you want Amazon Lex to return the intent data to the client. </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FulfillmentActivity {
    /// <p> A description of the Lambda function that is run to fulfill the intent. </p>
    #[serde(rename = "codeHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_hook: Option<CodeHook>,
    /// <p> How the intent should be fulfilled, either by running a Lambda function or by returning the slot data to the client application. </p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotAliasRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the bot alias. The name is case sensitive.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotAliasResponse {
    /// <p>The name of the bot that the alias points to.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The version of the bot that the alias points to.</p>
    #[serde(rename = "botVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    /// <p>Checksum of the bot alias.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The date that the bot alias was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot alias.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the bot alias.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotAliasesRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The maximum number of aliases to return in the response. The default is 50. . </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Substring to match in bot alias names. An alias will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[serde(rename = "nameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>A pagination token for fetching the next page of aliases. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of aliases, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotAliasesResponse {
    /// <p>An array of <code>BotAliasMetadata</code> objects, each describing a bot alias.</p>
    #[serde(rename = "BotAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_aliases: Option<Vec<BotAliasMetadata>>,
    /// <p>A pagination token for fetching next page of aliases. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of aliases, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotChannelAssociationRequest {
    /// <p>An alias pointing to the specific version of the Amazon Lex bot to which this association is being made.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the Amazon Lex bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the association between the bot and the channel. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotChannelAssociationResponse {
    /// <p>An alias pointing to the specific version of the Amazon Lex bot to which this association is being made.</p>
    #[serde(rename = "botAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias: Option<String>,
    /// <p>Provides information that the messaging platform needs to communicate with the Amazon Lex bot.</p>
    #[serde(rename = "botConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the Amazon Lex bot.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The date that the association between the bot and the channel was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the association between the bot and the channel.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the reason that it failed to create the association.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the association between the bot and the channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the bot channel. </p> <ul> <li> <p> <code>CREATED</code> - The channel has been created and is ready for use.</p> </li> <li> <p> <code>IN_PROGRESS</code> - Channel creation is in progress.</p> </li> <li> <p> <code>FAILED</code> - There was an error creating the channel. For information about the reason for the failure, see the <code>failureReason</code> field.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the messaging platform.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotChannelAssociationsRequest {
    /// <p>An alias pointing to the specific version of the Amazon Lex bot to which this association is being made.</p>
    #[serde(rename = "botAlias")]
    pub bot_alias: String,
    /// <p>The name of the Amazon Lex bot in the association.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The maximum number of associations to return in the response. The default is 50. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Substring to match in channel association names. An association will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz." To return all bot channel associations, use a hyphen ("-") as the <code>nameContains</code> parameter.</p>
    #[serde(rename = "nameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>A pagination token for fetching the next page of associations. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of associations, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotChannelAssociationsResponse {
    /// <p>An array of objects, one for each association, that provides information about the Amazon Lex bot and its association with the channel. </p>
    #[serde(rename = "botChannelAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_channel_associations: Option<Vec<BotChannelAssociation>>,
    /// <p>A pagination token that fetches the next page of associations. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of associations, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotRequest {
    /// <p>The name of the bot. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version or alias of the bot.</p>
    #[serde(rename = "versionOrAlias")]
    pub version_or_alias: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotResponse {
    /// <p>The message that Amazon Lex returns when the user elects to end the conversation without completing it. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "abortStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_statement: Option<Statement>,
    /// <p>Checksum of the bot used to identify a specific revision of the bot's <code>$LATEST</code> version.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service, you must specify whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to the Children's Online Privacy Protection Act (COPPA) by specifying <code>true</code> or <code>false</code> in the <code>childDirected</code> field. By specifying <code>true</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. By specifying <code>false</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is not</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. You may not specify a default value for the <code>childDirected</code> field that does not accurately reflect whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.</p> <p>If your use of Amazon Lex relates to a website, program, or other application that is directed in whole or in part, to children under age 13, you must obtain any required verifiable parental consent under COPPA. For information regarding the use of Amazon Lex in connection with websites, programs, or other applications that are directed or targeted, in whole or in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a> </p>
    #[serde(rename = "childDirected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_directed: Option<bool>,
    /// <p>The message Amazon Lex uses when it doesn't understand the user's request. For more information, see <a>PutBot</a>. </p>
    #[serde(rename = "clarificationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_prompt: Option<Prompt>,
    /// <p>The date that the bot was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex explains why it failed to build the bot.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The maximum time in seconds that Amazon Lex retains the data gathered in a conversation. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_ttl_in_seconds: Option<i64>,
    /// <p>An array of <code>intent</code> objects. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    /// <p>The date that the bot was updated. When you create a resource, the creation date and last updated date are the same. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p> The target locale for the bot. </p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The name of the bot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the bot. If the bot is ready to run, the status is <code>READY</code>. If there was a problem with building the bot, the status is <code>FAILED</code> and the <code>failureReason</code> explains why the bot did not build. If the bot was saved but not built, the status is <code>NOT BUILT</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version of the bot. For a new bot, the version is always <code>$LATEST</code>.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotVersionsRequest {
    /// <p>The maximum number of bot versions to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the bot for which versions should be returned.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A pagination token for fetching the next page of bot versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotVersionsResponse {
    /// <p>An array of <code>BotMetadata</code> objects, one for each numbered version of the bot plus one for the <code>$LATEST</code> version.</p>
    #[serde(rename = "bots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bots: Option<Vec<BotMetadata>>,
    /// <p>A pagination token for fetching the next page of bot versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBotsRequest {
    /// <p>The maximum number of bots to return in the response that the request will return. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Substring to match in bot names. A bot will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[serde(rename = "nameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>A pagination token that fetches the next page of bots. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of bots, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBotsResponse {
    /// <p>An array of <code>botMetadata</code> objects, with one entry for each bot. </p>
    #[serde(rename = "bots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bots: Option<Vec<BotMetadata>>,
    /// <p>If the response is truncated, it includes a pagination token that you can specify in your next request to fetch the next page of bots. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBuiltinIntentRequest {
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "signature")]
    pub signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBuiltinIntentResponse {
    /// <p>The unique identifier for a built-in intent.</p>
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// <p>An array of <code>BuiltinIntentSlot</code> objects, one entry for each slot type in the intent.</p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<BuiltinIntentSlot>>,
    /// <p>A list of locales that the intent supports.</p>
    #[serde(rename = "supportedLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locales: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBuiltinIntentsRequest {
    /// <p>A list of locales that the intent supports.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of intents to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token that fetches the next page of intents. If this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, use the pagination token in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Substring to match in built-in intent signatures. An intent will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz." To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "signatureContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_contains: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBuiltinIntentsResponse {
    /// <p>An array of <code>builtinIntentMetadata</code> objects, one for each intent in the response.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<BuiltinIntentMetadata>>,
    /// <p>A pagination token that fetches the next page of intents. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, specify the pagination token in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBuiltinSlotTypesRequest {
    /// <p>A list of locales that the slot type supports.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of slot types, specify the pagination token in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Substring to match in built-in slot type signatures. A slot type will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[serde(rename = "signatureContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_contains: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetBuiltinSlotTypesResponse {
    /// <p>If the response is truncated, the response includes a pagination token that you can use in your next request to fetch the next page of slot types.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>BuiltInSlotTypeMetadata</code> objects, one entry for each slot type returned.</p>
    #[serde(rename = "slotTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_types: Option<Vec<BuiltinSlotTypeMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportRequest {
    /// <p>The format of the exported data.</p>
    #[serde(rename = "exportType")]
    pub export_type: String,
    /// <p>The name of the bot to export.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The type of resource to export. </p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// <p>The version of the bot to export.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetExportResponse {
    /// <p><p>The status of the export. </p> <ul> <li> <p> <code>IN_PROGRESS</code> - The export is in progress.</p> </li> <li> <p> <code>READY</code> - The export is complete.</p> </li> <li> <p> <code>FAILED</code> - The export could not be completed.</p> </li> </ul></p>
    #[serde(rename = "exportStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    /// <p>The format of the exported data.</p>
    #[serde(rename = "exportType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_type: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the reason that it failed to export the resource.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the bot being exported.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of the exported resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>An S3 pre-signed URL that provides the location of the exported resource. The exported resource is a ZIP archive that contains the exported resource in JSON format. The structure of the archive may change. Your code should not rely on the archive structure.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The version of the bot being exported.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetImportRequest {
    /// <p>The identifier of the import job information to return.</p>
    #[serde(rename = "importId")]
    pub import_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetImportResponse {
    /// <p>A timestamp for the date and time that the import job was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A string that describes why an import job failed to complete.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<Vec<String>>,
    /// <p>The identifier for the specific import job.</p>
    #[serde(rename = "importId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    /// <p>The status of the import job. If the status is <code>FAILED</code>, you can get the reason for the failure from the <code>failureReason</code> field.</p>
    #[serde(rename = "importStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    /// <p>The action taken when there was a conflict between an existing resource and a resource in the import file.</p>
    #[serde(rename = "mergeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,
    /// <p>The name given to the import job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of resource imported.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntentRequest {
    /// <p>The name of the intent. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the intent.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetIntentResponse {
    /// <p>Checksum of the intent.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>After the Lambda function specified in the <code>fulfillmentActivity</code> element fulfills the intent, Amazon Lex conveys this statement to the user.</p>
    #[serde(rename = "conclusionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion_statement: Option<Statement>,
    /// <p>If defined in the bot, Amazon Lex uses prompt to confirm the intent before fulfilling the user's request. For more information, see <a>PutIntent</a>. </p>
    #[serde(rename = "confirmationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_prompt: Option<Prompt>,
    /// <p>The date that the intent was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the intent.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If defined in the bot, Amazon Amazon Lex invokes this Lambda function for each user input. For more information, see <a>PutIntent</a>. </p>
    #[serde(rename = "dialogCodeHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<CodeHook>,
    /// <p>If defined in the bot, Amazon Lex uses this prompt to solicit additional user activity after the intent is fulfilled. For more information, see <a>PutIntent</a>.</p>
    #[serde(rename = "followUpPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_prompt: Option<FollowUpPrompt>,
    /// <p>Describes how the intent is fulfilled. For more information, see <a>PutIntent</a>. </p>
    #[serde(rename = "fulfillmentActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_activity: Option<FulfillmentActivity>,
    /// <p>The date that the intent was updated. When you create a resource, the creation date and the last updated date are the same. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a built-in intent.</p>
    #[serde(rename = "parentIntentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    /// <p>If the user answers "no" to the question defined in <code>confirmationPrompt</code>, Amazon Lex responds with this statement to acknowledge that the intent was canceled. </p>
    #[serde(rename = "rejectionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_statement: Option<Statement>,
    /// <p>An array of sample utterances configured for the intent.</p>
    #[serde(rename = "sampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>An array of intent slots configured for the intent.</p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Slot>>,
    /// <p>The version of the intent.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntentVersionsRequest {
    /// <p>The maximum number of intent versions to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the intent for which versions should be returned.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A pagination token for fetching the next page of intent versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetIntentVersionsResponse {
    /// <p>An array of <code>IntentMetadata</code> objects, one for each numbered version of the intent plus one for the <code>$LATEST</code> version.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<IntentMetadata>>,
    /// <p>A pagination token for fetching the next page of intent versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntentsRequest {
    /// <p>The maximum number of intents to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Substring to match in intent names. An intent will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[serde(rename = "nameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>A pagination token that fetches the next page of intents. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetIntentsResponse {
    /// <p>An array of <code>Intent</code> objects. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<IntentMetadata>>,
    /// <p>If the response is truncated, the response includes a pagination token that you can specify in your next request to fetch the next page of intents. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSlotTypeRequest {
    /// <p>The name of the slot type. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the slot type. </p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSlotTypeResponse {
    /// <p>Checksum of the <code>$LATEST</code> version of the slot type.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The date that the slot type was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the slot type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of <code>EnumerationValue</code> objects that defines the values that the slot type can take.</p>
    #[serde(rename = "enumerationValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration_values: Option<Vec<EnumerationValue>>,
    /// <p>The date that the slot type was updated. When you create a resource, the creation date and last update date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The strategy that Amazon Lex uses to determine the value of the slot. For more information, see <a>PutSlotType</a>.</p>
    #[serde(rename = "valueSelectionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_strategy: Option<String>,
    /// <p>The version of the slot type.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSlotTypeVersionsRequest {
    /// <p>The maximum number of slot type versions to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the slot type for which versions should be returned.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A pagination token for fetching the next page of slot type versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSlotTypeVersionsResponse {
    /// <p>A pagination token for fetching the next page of slot type versions. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of versions, specify the pagination token in the next request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>SlotTypeMetadata</code> objects, one for each numbered version of the slot type plus one for the <code>$LATEST</code> version.</p>
    #[serde(rename = "slotTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_types: Option<Vec<SlotTypeMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSlotTypesRequest {
    /// <p>The maximum number of slot types to return in the response. The default is 10.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Substring to match in slot type names. A slot type will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
    #[serde(rename = "nameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>A pagination token that fetches the next page of slot types. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch next page of slot types, specify the pagination token in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSlotTypesResponse {
    /// <p>If the response is truncated, it includes a pagination token that you can specify in your next request to fetch the next page of slot types.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects, one for each slot type, that provides information such as the name of the slot type, the version, and a description.</p>
    #[serde(rename = "slotTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_types: Option<Vec<SlotTypeMetadata>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUtterancesViewRequest {
    /// <p>The name of the bot for which utterance information should be returned.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    #[serde(rename = "botVersions")]
    pub bot_versions: Vec<String>,
    /// <p>To return utterances that were recognized and handled, use<code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    #[serde(rename = "statusType")]
    pub status_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetUtterancesViewResponse {
    /// <p>The name of the bot for which utterance information was returned.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>An array of <a>UtteranceList</a> objects, each containing a list of <a>UtteranceData</a> objects describing the utterances that were processed by your bot. The response contains a maximum of 100 <code>UtteranceData</code> objects for each version.</p>
    #[serde(rename = "utterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterances: Option<Vec<UtteranceList>>,
}

/// <p>Identifies the specific version of an intent.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// <p>The name of the intent.</p>
    #[serde(rename = "intentName")]
    pub intent_name: String,
    /// <p>The version of the intent.</p>
    #[serde(rename = "intentVersion")]
    pub intent_version: String,
}

/// <p>Provides information about an intent.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IntentMetadata {
    /// <p>The date that the intent was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the intent.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the intent was updated. When you create an intent, the creation date and last updated date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the intent.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>The message object that provides the message text and its type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// <p>The text of the message.</p>
    #[serde(rename = "content")]
    pub content: String,
    /// <p>The content type of the message string.</p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p>Identifies the message group that the message belongs to. When a group is assigned to a message, Amazon Lex returns one message from each group in the response.</p>
    #[serde(rename = "groupNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_number: Option<i64>,
}

/// <p>Obtains information from the user. To define a prompt, provide one or more messages and specify the number of attempts to get information from the user. If you provide more than one message, Amazon Lex chooses one of the messages to use to prompt the user. For more information, see <a>how-it-works</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    /// <p>The number of times to prompt the user for information.</p>
    #[serde(rename = "maxAttempts")]
    pub max_attempts: i64,
    /// <p>An array of objects, each of which provides a message string and its type. You can specify the message string in plain text or in Speech Synthesis Markup Language (SSML).</p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
    /// <p>A response card. Amazon Lex uses this prompt at runtime, in the <code>PostText</code> API response. It substitutes session attributes and slot values for placeholders in the response card. For more information, see <a>ex-resp-card</a>. </p>
    #[serde(rename = "responseCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_card: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutBotAliasRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The version of the bot.</p>
    #[serde(rename = "botVersion")]
    pub bot_version: String,
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version.</p> <p>When you create a new bot alias, leave the <code>checksum</code> field blank. If you specify a checksum you get a <code>BadRequestException</code> exception.</p> <p>When you want to update a bot alias, set the <code>checksum</code> field to the checksum of the most recent revision of the <code>$LATEST</code> version. If you don't specify the <code> checksum</code> field, or if the checksum does not match the <code>$LATEST</code> version, you get a <code>PreconditionFailedException</code> exception.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>A description of the alias.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the alias. The name is <i>not</i> case sensitive.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutBotAliasResponse {
    /// <p>The name of the bot that the alias points to.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>The version of the bot that the alias points to.</p>
    #[serde(rename = "botVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    /// <p>The checksum for the current version of the alias.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The date that the bot alias was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the alias.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the alias.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutBotRequest {
    /// <p>When Amazon Lex can't understand the user's input in context, it tries to elicit the information a few times. After that, Amazon Lex sends the message defined in <code>abortStatement</code> to the user, and then aborts the conversation. To set the number of retries, use the <code>valueElicitationPrompt</code> field for the slot type. </p> <p>For example, in a pizza ordering bot, Amazon Lex might ask a user "What type of crust would you like?" If the user's response is not one of the expected responses (for example, "thin crust, "deep dish," etc.), Amazon Lex tries to elicit a correct response a few more times. </p> <p>For example, in a pizza ordering application, <code>OrderPizza</code> might be one of the intents. This intent might require the <code>CrustType</code> slot. You specify the <code>valueElicitationPrompt</code> field when you create the <code>CrustType</code> slot.</p>
    #[serde(rename = "abortStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_statement: Option<Statement>,
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version.</p> <p>When you create a new bot, leave the <code>checksum</code> field blank. If you specify a checksum you get a <code>BadRequestException</code> exception.</p> <p>When you want to update a bot, set the <code>checksum</code> field to the checksum of the most recent revision of the <code>$LATEST</code> version. If you don't specify the <code> checksum</code> field, or if the checksum does not match the <code>$LATEST</code> version, you get a <code>PreconditionFailedException</code> exception.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service, you must specify whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to the Children's Online Privacy Protection Act (COPPA) by specifying <code>true</code> or <code>false</code> in the <code>childDirected</code> field. By specifying <code>true</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. By specifying <code>false</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is not</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. You may not specify a default value for the <code>childDirected</code> field that does not accurately reflect whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.</p> <p>If your use of Amazon Lex relates to a website, program, or other application that is directed in whole or in part, to children under age 13, you must obtain any required verifiable parental consent under COPPA. For information regarding the use of Amazon Lex in connection with websites, programs, or other applications that are directed or targeted, in whole or in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a> </p>
    #[serde(rename = "childDirected")]
    pub child_directed: bool,
    /// <p>When Amazon Lex doesn't understand the user's intent, it uses this message to get clarification. To specify how many times Amazon Lex should repeate the clarification prompt, use the <code>maxAttempts</code> field. If Amazon Lex still doesn't understand, it sends the message in the <code>abortStatement</code> field. </p> <p>When you create a clarification prompt, make sure that it suggests the correct response from the user. for example, for a bot that orders pizza and drinks, you might create this clarification prompt: "What would you like to do? You can say 'Order a pizza' or 'Order a drink.'"</p>
    #[serde(rename = "clarificationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_prompt: Option<Prompt>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The maximum time in seconds that Amazon Lex retains the data gathered in a conversation.</p> <p>A user interaction session remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p> <p>For example, suppose that a user chooses the OrderPizza intent, but gets sidetracked halfway through placing an order. If the user doesn't complete the order within the specified time, Amazon Lex discards the slot information that it gathered, and the user must start over.</p> <p>If you don't include the <code>idleSessionTTLInSeconds</code> element in a <code>PutBot</code> operation request, Amazon Lex uses the default value. This is also true if the request replaces an existing bot.</p> <p>The default is 300 seconds (5 minutes).</p>
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_ttl_in_seconds: Option<i64>,
    /// <p>An array of <code>Intent</code> objects. Each intent represents a command that a user can express. For example, a pizza ordering bot might support an OrderPizza intent. For more information, see <a>how-it-works</a>.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    /// <p> Specifies the target locale for the bot. Any intent used in the bot must be compatible with the locale of the bot. </p> <p>The default is <code>en-US</code>.</p>
    #[serde(rename = "locale")]
    pub locale: String,
    /// <p>The name of the bot. The name is <i>not</i> case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>If you set the <code>processBehavior</code> element to <code>BUILD</code>, Amazon Lex builds the bot so that it can be run. If you set the element to <code>SAVE</code> Amazon Lex saves the bot, but doesn't build it. </p> <p>If you don't specify this value, the default value is <code>BUILD</code>.</p>
    #[serde(rename = "processBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_behavior: Option<String>,
    /// <p>The Amazon Polly voice ID that you want Amazon Lex to use for voice interactions with the user. The locale configured for the voice must match the locale of the bot. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/voicelist.html">Available Voices</a> in the <i>Amazon Polly Developer Guide</i>.</p>
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutBotResponse {
    /// <p>The message that Amazon Lex uses to abort a conversation. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "abortStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_statement: Option<Statement>,
    /// <p>Checksum of the bot that you created.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>For each Amazon Lex bot created with the Amazon Lex Model Building Service, you must specify whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to the Children's Online Privacy Protection Act (COPPA) by specifying <code>true</code> or <code>false</code> in the <code>childDirected</code> field. By specifying <code>true</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. By specifying <code>false</code> in the <code>childDirected</code> field, you confirm that your use of Amazon Lex <b>is not</b> related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA. You may not specify a default value for the <code>childDirected</code> field that does not accurately reflect whether your use of Amazon Lex is related to a website, program, or other application that is directed or targeted, in whole or in part, to children under age 13 and subject to COPPA.</p> <p>If your use of Amazon Lex relates to a website, program, or other application that is directed in whole or in part, to children under age 13, you must obtain any required verifiable parental consent under COPPA. For information regarding the use of Amazon Lex in connection with websites, programs, or other applications that are directed or targeted, in whole or in part, to children under age 13, see the <a href="https://aws.amazon.com/lex/faqs#data-security">Amazon Lex FAQ.</a> </p>
    #[serde(rename = "childDirected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_directed: Option<bool>,
    /// <p> The prompts that Amazon Lex uses when it doesn't understand the user's intent. For more information, see <a>PutBot</a>. </p>
    #[serde(rename = "clarificationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_prompt: Option<Prompt>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>The date that the bot was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If <code>status</code> is <code>FAILED</code>, Amazon Lex provides the reason that it failed to build the bot.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The maximum length of time that Amazon Lex retains the data gathered in a conversation. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_ttl_in_seconds: Option<i64>,
    /// <p>An array of <code>Intent</code> objects. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "intents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    /// <p>The date that the bot was updated. When you create a resource, the creation date and last updated date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p> The target locale for the bot. </p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The name of the bot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> When you send a request to create a bot with <code>processBehavior</code> set to <code>BUILD</code>, Amazon Lex sets the <code>status</code> response element to <code>BUILDING</code>. After Amazon Lex builds the bot, it sets <code>status</code> to <code>READY</code>. If Amazon Lex can't build the bot, Amazon Lex sets <code>status</code> to <code>FAILED</code>. Amazon Lex returns the reason for the failure in the <code>failureReason</code> response element. </p> <p>When you set <code>processBehavior</code>to <code>SAVE</code>, Amazon Lex sets the status code to <code>NOT BUILT</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The version of the bot. For a new bot, the version is always <code>$LATEST</code>.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The Amazon Polly voice ID that Amazon Lex uses for voice interaction with the user. For more information, see <a>PutBot</a>.</p>
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutIntentRequest {
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version.</p> <p>When you create a new intent, leave the <code>checksum</code> field blank. If you specify a checksum you get a <code>BadRequestException</code> exception.</p> <p>When you want to update a intent, set the <code>checksum</code> field to the checksum of the most recent revision of the <code>$LATEST</code> version. If you don't specify the <code> checksum</code> field, or if the checksum does not match the <code>$LATEST</code> version, you get a <code>PreconditionFailedException</code> exception.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p><p> The statement that you want Amazon Lex to convey to the user after the intent is successfully fulfilled by the Lambda function. </p> <p>This element is relevant only if you provide a Lambda function in the <code>fulfillmentActivity</code>. If you return the intent to the client application, you can&#39;t specify this element.</p> <note> <p>The <code>followUpPrompt</code> and <code>conclusionStatement</code> are mutually exclusive. You can specify only one.</p> </note></p>
    #[serde(rename = "conclusionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion_statement: Option<Statement>,
    /// <p><p>Prompts the user to confirm the intent. This question should have a yes or no answer.</p> <p>Amazon Lex uses this prompt to ensure that the user acknowledges that the intent is ready for fulfillment. For example, with the <code>OrderPizza</code> intent, you might want to confirm that the order is correct before placing it. For other intents, such as intents that simply respond to user questions, you might not need to ask the user for confirmation before providing the information. </p> <note> <p>You you must provide both the <code>rejectionStatement</code> and the <code>confirmationPrompt</code>, or neither.</p> </note></p>
    #[serde(rename = "confirmationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_prompt: Option<Prompt>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>A description of the intent.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p> Specifies a Lambda function to invoke for each user input. You can invoke this Lambda function to personalize user interaction. </p> <p>For example, suppose your bot determines that the user is John. Your Lambda function might retrieve John's information from a backend database and prepopulate some of the values. For example, if you find that John is gluten intolerant, you might set the corresponding intent slot, <code>GlutenIntolerant</code>, to true. You might find John's phone number and set the corresponding session attribute. </p>
    #[serde(rename = "dialogCodeHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<CodeHook>,
    /// <p>Amazon Lex uses this prompt to solicit additional activity after fulfilling an intent. For example, after the <code>OrderPizza</code> intent is fulfilled, you might prompt the user to order a drink.</p> <p>The action that Amazon Lex takes depends on the user's response, as follows:</p> <ul> <li> <p>If the user says "Yes" it responds with the clarification prompt that is configured for the bot.</p> </li> <li> <p>if the user says "Yes" and continues with an utterance that triggers an intent it starts a conversation for the intent.</p> </li> <li> <p>If the user says "No" it responds with the rejection statement configured for the the follow-up prompt.</p> </li> <li> <p>If it doesn't recognize the utterance it repeats the follow-up prompt again.</p> </li> </ul> <p>The <code>followUpPrompt</code> field and the <code>conclusionStatement</code> field are mutually exclusive. You can specify only one. </p>
    #[serde(rename = "followUpPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_prompt: Option<FollowUpPrompt>,
    /// <p>Required. Describes how the intent is fulfilled. For example, after a user provides all of the information for a pizza order, <code>fulfillmentActivity</code> defines how the bot places an order with a local pizza store. </p> <p> You might configure Amazon Lex to return all of the intent information to the client application, or direct it to invoke a Lambda function that can process the intent (for example, place an order with a pizzeria). </p>
    #[serde(rename = "fulfillmentActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_activity: Option<FulfillmentActivity>,
    /// <p>The name of the intent. The name is <i>not</i> case sensitive. </p> <p>The name can't match a built-in intent name, or a built-in intent name with "AMAZON." removed. For example, because there is a built-in intent called <code>AMAZON.HelpIntent</code>, you can't create a custom intent called <code>HelpIntent</code>.</p> <p>For a list of built-in intents, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for the built-in intent to base this intent on. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "parentIntentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    /// <p><p>When the user answers &quot;no&quot; to the question defined in <code>confirmationPrompt</code>, Amazon Lex responds with this statement to acknowledge that the intent was canceled. </p> <note> <p>You must provide both the <code>rejectionStatement</code> and the <code>confirmationPrompt</code>, or neither.</p> </note></p>
    #[serde(rename = "rejectionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_statement: Option<Statement>,
    /// <p>An array of utterances (strings) that a user might say to signal the intent. For example, "I want {PizzaSize} pizza", "Order {Quantity} {PizzaSize} pizzas". </p> <p>In each utterance, a slot name is enclosed in curly braces. </p>
    #[serde(rename = "sampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>An array of intent slots. At runtime, Amazon Lex elicits required slot values from the user using prompts defined in the slots. For more information, see <a>how-it-works</a>. </p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Slot>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutIntentResponse {
    /// <p>Checksum of the <code>$LATEST</code>version of the intent created or updated.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>After the Lambda function specified in the<code>fulfillmentActivity</code>intent fulfills the intent, Amazon Lex conveys this statement to the user.</p>
    #[serde(rename = "conclusionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion_statement: Option<Statement>,
    /// <p>If defined in the intent, Amazon Lex prompts the user to confirm the intent before fulfilling it.</p>
    #[serde(rename = "confirmationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_prompt: Option<Prompt>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>The date that the intent was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the intent.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If defined in the intent, Amazon Lex invokes this Lambda function for each user input.</p>
    #[serde(rename = "dialogCodeHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<CodeHook>,
    /// <p>If defined in the intent, Amazon Lex uses this prompt to solicit additional user activity after the intent is fulfilled.</p>
    #[serde(rename = "followUpPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_prompt: Option<FollowUpPrompt>,
    /// <p>If defined in the intent, Amazon Lex invokes this Lambda function to fulfill the intent after the user provides all of the information required by the intent.</p>
    #[serde(rename = "fulfillmentActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_activity: Option<FulfillmentActivity>,
    /// <p>The date that the intent was updated. When you create a resource, the creation date and last update dates are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for the built-in intent that this intent is based on.</p>
    #[serde(rename = "parentIntentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    /// <p>If the user answers "no" to the question defined in <code>confirmationPrompt</code> Amazon Lex responds with this statement to acknowledge that the intent was canceled. </p>
    #[serde(rename = "rejectionStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_statement: Option<Statement>,
    /// <p> An array of sample utterances that are configured for the intent. </p>
    #[serde(rename = "sampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>An array of intent slots that are configured for the intent.</p>
    #[serde(rename = "slots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Slot>>,
    /// <p>The version of the intent. For a new intent, the version is always <code>$LATEST</code>.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutSlotTypeRequest {
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version.</p> <p>When you create a new slot type, leave the <code>checksum</code> field blank. If you specify a checksum you get a <code>BadRequestException</code> exception.</p> <p>When you want to update a slot type, set the <code>checksum</code> field to the checksum of the most recent revision of the <code>$LATEST</code> version. If you don't specify the <code> checksum</code> field, or if the checksum does not match the <code>$LATEST</code> version, you get a <code>PreconditionFailedException</code> exception.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>A description of the slot type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of <code>EnumerationValue</code> objects that defines the values that the slot type can take. Each value can have a list of <code>synonyms</code>, which are additional values that help train the machine learning model about the values that it resolves for a slot. </p> <p>When Amazon Lex resolves a slot value, it generates a resolution list that contains up to five possible values for the slot. If you are using a Lambda function, this resolution list is passed to the function. If you are not using a Lambda function you can choose to return the value that the user entered or the first value in the resolution list as the slot value. The <code>valueSelectionStrategy</code> field indicates the option to use. </p>
    #[serde(rename = "enumerationValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration_values: Option<Vec<EnumerationValue>>,
    /// <p>The name of the slot type. The name is <i>not</i> case sensitive. </p> <p>The name can't match a built-in slot type name, or a built-in slot type name with "AMAZON." removed. For example, because there is a built-in slot type called <code>AMAZON.DATE</code>, you can't create a custom slot type called <code>DATE</code>.</p> <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Determines the slot resolution strategy that Amazon Lex uses to return slot type values. The field can be set to one of the following values:</p> <ul> <li> <p> <code>ORIGINAL_VALUE</code> - Returns the value entered by the user, if the user value is similar to the slot value.</p> </li> <li> <p> <code>TOP_RESOLUTION</code> - If there is a resolution list for the slot, return the first value in the resolution list as the slot type value. If there is no resolution list, null is returned.</p> </li> </ul> <p>If you don't specify the <code>valueSelectionStrategy</code>, the default is <code>ORIGINAL_VALUE</code>.</p>
    #[serde(rename = "valueSelectionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_strategy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutSlotTypeResponse {
    /// <p>Checksum of the <code>$LATEST</code> version of the slot type.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>The date that the slot type was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the slot type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A list of <code>EnumerationValue</code> objects that defines the values that the slot type can take.</p>
    #[serde(rename = "enumerationValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration_values: Option<Vec<EnumerationValue>>,
    /// <p>The date that the slot type was updated. When you create a slot type, the creation date and last update date are the same.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The slot resolution strategy that Amazon Lex uses to determine the value of the slot. For more information, see <a>PutSlotType</a>.</p>
    #[serde(rename = "valueSelectionStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_strategy: Option<String>,
    /// <p>The version of the slot type. For a new slot type, the version is always <code>$LATEST</code>. </p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Describes the resource that refers to the resource that you are attempting to delete. This object is returned as part of the <code>ResourceInUseException</code> exception. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResourceReference {
    /// <p>The name of the resource that is using the resource that you are trying to delete.</p>
    pub name: Option<String>,
    /// <p>The version of the resource that is using the resource that you are trying to delete.</p>
    pub version: Option<String>,
}

/// <p>Identifies the version of a specific slot.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    /// <p>A description of the slot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the slot.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p> Directs Lex the order in which to elicit this slot value from the user. For example, if the intent has two slots with priorities 1 and 2, AWS Lex first elicits a value for the slot with priority 1.</p> <p>If multiple slots share the same priority, the order in which Lex elicits values is arbitrary.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p> A set of possible responses for the slot type used by text-based clients. A user chooses an option from the response card, instead of using text to reply. </p>
    #[serde(rename = "responseCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_card: Option<String>,
    /// <p> If you know a specific pattern with which users might respond to an Amazon Lex request for a slot value, you can provide those utterances to improve accuracy. This is optional. In most cases, Amazon Lex is capable of understanding user utterances. </p>
    #[serde(rename = "sampleUtterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<String>>,
    /// <p>Specifies whether the slot is required or optional. </p>
    #[serde(rename = "slotConstraint")]
    pub slot_constraint: String,
    /// <p>The type of the slot, either a custom slot type that you defined or one of the built-in slot types.</p>
    #[serde(rename = "slotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type: Option<String>,
    /// <p>The version of the slot type.</p>
    #[serde(rename = "slotTypeVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_version: Option<String>,
    /// <p>The prompt that Amazon Lex uses to elicit the slot value from the user.</p>
    #[serde(rename = "valueElicitationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_elicitation_prompt: Option<Prompt>,
}

/// <p>Provides information about a slot type..</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SlotTypeMetadata {
    /// <p>The date that the slot type was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>A description of the slot type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date that the slot type was updated. When you create a resource, the creation date and last updated date are the same. </p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The version of the slot type.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartImportRequest {
    /// <p><p>Specifies the action that the <code>StartImport</code> operation should take when there is an existing resource with the same name.</p> <ul> <li> <p>FAIL<em>ON</em>CONFLICT - The import operation is stopped on the first conflict between a resource in the import file and an existing resource. The name of the resource causing the conflict is in the <code>failureReason</code> field of the response to the <code>GetImport</code> operation.</p> <p>OVERWRITE_LATEST - The import operation proceeds even if there is a conflict with an existing resource. The $LASTEST version of the existing resource is overwritten with the data from the import file.</p> </li> </ul></p>
    #[serde(rename = "mergeStrategy")]
    pub merge_strategy: String,
    /// <p>A zip archive in binary format. The archive should contain one file, a JSON file containing the resource to import. The resource should match the type specified in the <code>resourceType</code> field.</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: Vec<u8>,
    /// <p><p>Specifies the type of resource to export. Each resource also exports any resources that it depends on. </p> <ul> <li> <p>A bot exports dependent intents.</p> </li> <li> <p>An intent exports dependent slot types.</p> </li> </ul></p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartImportResponse {
    /// <p>A timestamp for the date and time that the import job was requested.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier for the specific import job.</p>
    #[serde(rename = "importId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    /// <p>The status of the import job. If the status is <code>FAILED</code>, you can get the reason for the failure using the <code>GetImport</code> operation.</p>
    #[serde(rename = "importStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    /// <p>The action to take when there is a merge conflict.</p>
    #[serde(rename = "mergeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,
    /// <p>The name given to the import job.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of resource to import.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>A collection of messages that convey information to the user. At runtime, Amazon Lex selects the message to convey. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    /// <p>A collection of message objects.</p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
    /// <p> At runtime, if the client is using the <a href="http://docs.aws.amazon.com/lex/latest/dg/API_runtime_PostText.html">PostText</a> API, Amazon Lex includes the response card in the response. It substitutes all of the session attributes and slot values for placeholders in the response card. </p>
    #[serde(rename = "responseCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_card: Option<String>,
}

/// <p>Provides information about a single utterance that was made to your bot. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UtteranceData {
    /// <p>The number of times that the utterance was processed.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The total number of individuals that used the utterance.</p>
    #[serde(rename = "distinctUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_users: Option<i64>,
    /// <p>The date that the utterance was first recorded.</p>
    #[serde(rename = "firstUtteredDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_uttered_date: Option<f64>,
    /// <p>The date that the utterance was last recorded.</p>
    #[serde(rename = "lastUtteredDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_uttered_date: Option<f64>,
    /// <p>The text that was entered by the user or the text representation of an audio clip.</p>
    #[serde(rename = "utteranceString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_string: Option<String>,
}

/// <p>Provides a list of utterances that have been made to a specific version of your bot. The list contains a maximum of 100 utterances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UtteranceList {
    /// <p>The version of the bot that processed the list.</p>
    #[serde(rename = "botVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    /// <p>One or more <a>UtteranceData</a> objects that contain information about the utterances that have been made to a bot. The maximum number of object is 100.</p>
    #[serde(rename = "utterances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterances: Option<Vec<UtteranceData>>,
}

/// Errors returned by CreateBotVersion
#[derive(Debug, PartialEq)]
pub enum CreateBotVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateBotVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateBotVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateBotVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateBotVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return CreateBotVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateBotVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateBotVersionError::NotFound(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return CreateBotVersionError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateBotVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateBotVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBotVersionError {
    fn from(err: serde_json::error::Error) -> CreateBotVersionError {
        CreateBotVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBotVersionError {
    fn from(err: CredentialsError) -> CreateBotVersionError {
        CreateBotVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBotVersionError {
    fn from(err: HttpDispatchError) -> CreateBotVersionError {
        CreateBotVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBotVersionError {
    fn from(err: io::Error) -> CreateBotVersionError {
        CreateBotVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBotVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBotVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateBotVersionError::BadRequest(ref cause) => cause,
            CreateBotVersionError::Conflict(ref cause) => cause,
            CreateBotVersionError::InternalFailure(ref cause) => cause,
            CreateBotVersionError::LimitExceeded(ref cause) => cause,
            CreateBotVersionError::NotFound(ref cause) => cause,
            CreateBotVersionError::PreconditionFailed(ref cause) => cause,
            CreateBotVersionError::Validation(ref cause) => cause,
            CreateBotVersionError::Credentials(ref err) => err.description(),
            CreateBotVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBotVersionError::ParseError(ref cause) => cause,
            CreateBotVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateIntentVersion
#[derive(Debug, PartialEq)]
pub enum CreateIntentVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateIntentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateIntentVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateIntentVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateIntentVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return CreateIntentVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateIntentVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateIntentVersionError::NotFound(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return CreateIntentVersionError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateIntentVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateIntentVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateIntentVersionError {
    fn from(err: serde_json::error::Error) -> CreateIntentVersionError {
        CreateIntentVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIntentVersionError {
    fn from(err: CredentialsError) -> CreateIntentVersionError {
        CreateIntentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIntentVersionError {
    fn from(err: HttpDispatchError) -> CreateIntentVersionError {
        CreateIntentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateIntentVersionError {
    fn from(err: io::Error) -> CreateIntentVersionError {
        CreateIntentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateIntentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIntentVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateIntentVersionError::BadRequest(ref cause) => cause,
            CreateIntentVersionError::Conflict(ref cause) => cause,
            CreateIntentVersionError::InternalFailure(ref cause) => cause,
            CreateIntentVersionError::LimitExceeded(ref cause) => cause,
            CreateIntentVersionError::NotFound(ref cause) => cause,
            CreateIntentVersionError::PreconditionFailed(ref cause) => cause,
            CreateIntentVersionError::Validation(ref cause) => cause,
            CreateIntentVersionError::Credentials(ref err) => err.description(),
            CreateIntentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateIntentVersionError::ParseError(ref cause) => cause,
            CreateIntentVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateSlotTypeVersion
#[derive(Debug, PartialEq)]
pub enum CreateSlotTypeVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateSlotTypeVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateSlotTypeVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateSlotTypeVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateSlotTypeVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return CreateSlotTypeVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateSlotTypeVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return CreateSlotTypeVersionError::NotFound(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return CreateSlotTypeVersionError::PreconditionFailed(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateSlotTypeVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateSlotTypeVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateSlotTypeVersionError {
    fn from(err: serde_json::error::Error) -> CreateSlotTypeVersionError {
        CreateSlotTypeVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSlotTypeVersionError {
    fn from(err: CredentialsError) -> CreateSlotTypeVersionError {
        CreateSlotTypeVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSlotTypeVersionError {
    fn from(err: HttpDispatchError) -> CreateSlotTypeVersionError {
        CreateSlotTypeVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSlotTypeVersionError {
    fn from(err: io::Error) -> CreateSlotTypeVersionError {
        CreateSlotTypeVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSlotTypeVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSlotTypeVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateSlotTypeVersionError::BadRequest(ref cause) => cause,
            CreateSlotTypeVersionError::Conflict(ref cause) => cause,
            CreateSlotTypeVersionError::InternalFailure(ref cause) => cause,
            CreateSlotTypeVersionError::LimitExceeded(ref cause) => cause,
            CreateSlotTypeVersionError::NotFound(ref cause) => cause,
            CreateSlotTypeVersionError::PreconditionFailed(ref cause) => cause,
            CreateSlotTypeVersionError::Validation(ref cause) => cause,
            CreateSlotTypeVersionError::Credentials(ref err) => err.description(),
            CreateSlotTypeVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSlotTypeVersionError::ParseError(ref cause) => cause,
            CreateSlotTypeVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBot
#[derive(Debug, PartialEq)]
pub enum DeleteBotError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBotError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBotError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteBotError::BadRequest(String::from(error_message))
                }
                "ConflictException" => return DeleteBotError::Conflict(String::from(error_message)),
                "InternalFailureException" => {
                    return DeleteBotError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteBotError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return DeleteBotError::NotFound(String::from(error_message)),
                "ResourceInUseException" => {
                    return DeleteBotError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBotError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBotError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBotError {
    fn from(err: serde_json::error::Error) -> DeleteBotError {
        DeleteBotError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBotError {
    fn from(err: CredentialsError) -> DeleteBotError {
        DeleteBotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBotError {
    fn from(err: HttpDispatchError) -> DeleteBotError {
        DeleteBotError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBotError {
    fn from(err: io::Error) -> DeleteBotError {
        DeleteBotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBotError {
    fn description(&self) -> &str {
        match *self {
            DeleteBotError::BadRequest(ref cause) => cause,
            DeleteBotError::Conflict(ref cause) => cause,
            DeleteBotError::InternalFailure(ref cause) => cause,
            DeleteBotError::LimitExceeded(ref cause) => cause,
            DeleteBotError::NotFound(ref cause) => cause,
            DeleteBotError::ResourceInUse(ref cause) => cause,
            DeleteBotError::Validation(ref cause) => cause,
            DeleteBotError::Credentials(ref err) => err.description(),
            DeleteBotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBotError::ParseError(ref cause) => cause,
            DeleteBotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBotAlias
#[derive(Debug, PartialEq)]
pub enum DeleteBotAliasError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBotAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteBotAliasError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteBotAliasError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteBotAliasError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteBotAliasError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteBotAliasError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteBotAliasError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBotAliasError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBotAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBotAliasError {
    fn from(err: serde_json::error::Error) -> DeleteBotAliasError {
        DeleteBotAliasError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBotAliasError {
    fn from(err: CredentialsError) -> DeleteBotAliasError {
        DeleteBotAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBotAliasError {
    fn from(err: HttpDispatchError) -> DeleteBotAliasError {
        DeleteBotAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBotAliasError {
    fn from(err: io::Error) -> DeleteBotAliasError {
        DeleteBotAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBotAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBotAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteBotAliasError::BadRequest(ref cause) => cause,
            DeleteBotAliasError::Conflict(ref cause) => cause,
            DeleteBotAliasError::InternalFailure(ref cause) => cause,
            DeleteBotAliasError::LimitExceeded(ref cause) => cause,
            DeleteBotAliasError::NotFound(ref cause) => cause,
            DeleteBotAliasError::ResourceInUse(ref cause) => cause,
            DeleteBotAliasError::Validation(ref cause) => cause,
            DeleteBotAliasError::Credentials(ref err) => err.description(),
            DeleteBotAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBotAliasError::ParseError(ref cause) => cause,
            DeleteBotAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBotChannelAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteBotChannelAssociationError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBotChannelAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBotChannelAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteBotChannelAssociationError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteBotChannelAssociationError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteBotChannelAssociationError::InternalFailure(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return DeleteBotChannelAssociationError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "NotFoundException" => {
                    return DeleteBotChannelAssociationError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBotChannelAssociationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBotChannelAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBotChannelAssociationError {
    fn from(err: serde_json::error::Error) -> DeleteBotChannelAssociationError {
        DeleteBotChannelAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBotChannelAssociationError {
    fn from(err: CredentialsError) -> DeleteBotChannelAssociationError {
        DeleteBotChannelAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBotChannelAssociationError {
    fn from(err: HttpDispatchError) -> DeleteBotChannelAssociationError {
        DeleteBotChannelAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBotChannelAssociationError {
    fn from(err: io::Error) -> DeleteBotChannelAssociationError {
        DeleteBotChannelAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBotChannelAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBotChannelAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteBotChannelAssociationError::BadRequest(ref cause) => cause,
            DeleteBotChannelAssociationError::Conflict(ref cause) => cause,
            DeleteBotChannelAssociationError::InternalFailure(ref cause) => cause,
            DeleteBotChannelAssociationError::LimitExceeded(ref cause) => cause,
            DeleteBotChannelAssociationError::NotFound(ref cause) => cause,
            DeleteBotChannelAssociationError::Validation(ref cause) => cause,
            DeleteBotChannelAssociationError::Credentials(ref err) => err.description(),
            DeleteBotChannelAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteBotChannelAssociationError::ParseError(ref cause) => cause,
            DeleteBotChannelAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBotVersion
#[derive(Debug, PartialEq)]
pub enum DeleteBotVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBotVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBotVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteBotVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteBotVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteBotVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteBotVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteBotVersionError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteBotVersionError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBotVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBotVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBotVersionError {
    fn from(err: serde_json::error::Error) -> DeleteBotVersionError {
        DeleteBotVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBotVersionError {
    fn from(err: CredentialsError) -> DeleteBotVersionError {
        DeleteBotVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBotVersionError {
    fn from(err: HttpDispatchError) -> DeleteBotVersionError {
        DeleteBotVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBotVersionError {
    fn from(err: io::Error) -> DeleteBotVersionError {
        DeleteBotVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBotVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBotVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteBotVersionError::BadRequest(ref cause) => cause,
            DeleteBotVersionError::Conflict(ref cause) => cause,
            DeleteBotVersionError::InternalFailure(ref cause) => cause,
            DeleteBotVersionError::LimitExceeded(ref cause) => cause,
            DeleteBotVersionError::NotFound(ref cause) => cause,
            DeleteBotVersionError::ResourceInUse(ref cause) => cause,
            DeleteBotVersionError::Validation(ref cause) => cause,
            DeleteBotVersionError::Credentials(ref err) => err.description(),
            DeleteBotVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBotVersionError::ParseError(ref cause) => cause,
            DeleteBotVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteIntent
#[derive(Debug, PartialEq)]
pub enum DeleteIntentError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteIntentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteIntentError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteIntentError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteIntentError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteIntentError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteIntentError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteIntentError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteIntentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteIntentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteIntentError {
    fn from(err: serde_json::error::Error) -> DeleteIntentError {
        DeleteIntentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIntentError {
    fn from(err: CredentialsError) -> DeleteIntentError {
        DeleteIntentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIntentError {
    fn from(err: HttpDispatchError) -> DeleteIntentError {
        DeleteIntentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIntentError {
    fn from(err: io::Error) -> DeleteIntentError {
        DeleteIntentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIntentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntentError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntentError::BadRequest(ref cause) => cause,
            DeleteIntentError::Conflict(ref cause) => cause,
            DeleteIntentError::InternalFailure(ref cause) => cause,
            DeleteIntentError::LimitExceeded(ref cause) => cause,
            DeleteIntentError::NotFound(ref cause) => cause,
            DeleteIntentError::ResourceInUse(ref cause) => cause,
            DeleteIntentError::Validation(ref cause) => cause,
            DeleteIntentError::Credentials(ref err) => err.description(),
            DeleteIntentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIntentError::ParseError(ref cause) => cause,
            DeleteIntentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteIntentVersion
#[derive(Debug, PartialEq)]
pub enum DeleteIntentVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteIntentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteIntentVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteIntentVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteIntentVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteIntentVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteIntentVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteIntentVersionError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteIntentVersionError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteIntentVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteIntentVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteIntentVersionError {
    fn from(err: serde_json::error::Error) -> DeleteIntentVersionError {
        DeleteIntentVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIntentVersionError {
    fn from(err: CredentialsError) -> DeleteIntentVersionError {
        DeleteIntentVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIntentVersionError {
    fn from(err: HttpDispatchError) -> DeleteIntentVersionError {
        DeleteIntentVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteIntentVersionError {
    fn from(err: io::Error) -> DeleteIntentVersionError {
        DeleteIntentVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteIntentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntentVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntentVersionError::BadRequest(ref cause) => cause,
            DeleteIntentVersionError::Conflict(ref cause) => cause,
            DeleteIntentVersionError::InternalFailure(ref cause) => cause,
            DeleteIntentVersionError::LimitExceeded(ref cause) => cause,
            DeleteIntentVersionError::NotFound(ref cause) => cause,
            DeleteIntentVersionError::ResourceInUse(ref cause) => cause,
            DeleteIntentVersionError::Validation(ref cause) => cause,
            DeleteIntentVersionError::Credentials(ref err) => err.description(),
            DeleteIntentVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteIntentVersionError::ParseError(ref cause) => cause,
            DeleteIntentVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSlotType
#[derive(Debug, PartialEq)]
pub enum DeleteSlotTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSlotTypeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteSlotTypeError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteSlotTypeError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteSlotTypeError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteSlotTypeError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteSlotTypeError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteSlotTypeError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteSlotTypeError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteSlotTypeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSlotTypeError {
    fn from(err: serde_json::error::Error) -> DeleteSlotTypeError {
        DeleteSlotTypeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSlotTypeError {
    fn from(err: CredentialsError) -> DeleteSlotTypeError {
        DeleteSlotTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSlotTypeError {
    fn from(err: HttpDispatchError) -> DeleteSlotTypeError {
        DeleteSlotTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSlotTypeError {
    fn from(err: io::Error) -> DeleteSlotTypeError {
        DeleteSlotTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSlotTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSlotTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteSlotTypeError::BadRequest(ref cause) => cause,
            DeleteSlotTypeError::Conflict(ref cause) => cause,
            DeleteSlotTypeError::InternalFailure(ref cause) => cause,
            DeleteSlotTypeError::LimitExceeded(ref cause) => cause,
            DeleteSlotTypeError::NotFound(ref cause) => cause,
            DeleteSlotTypeError::ResourceInUse(ref cause) => cause,
            DeleteSlotTypeError::Validation(ref cause) => cause,
            DeleteSlotTypeError::Credentials(ref err) => err.description(),
            DeleteSlotTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSlotTypeError::ParseError(ref cause) => cause,
            DeleteSlotTypeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteSlotTypeVersion
#[derive(Debug, PartialEq)]
pub enum DeleteSlotTypeVersionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// <p>The resource that you are attempting to delete is referred to by another resource. Use this information to remove references to the resource that you are trying to delete.</p> <p>The body of the exception contains a JSON object that describes the resource.</p> <p> <code>{ "resourceType": BOT | BOTALIAS | BOTCHANNEL | INTENT,</code> </p> <p> <code>"resourceReference": {</code> </p> <p> <code>"name": <i>string</i>, "version": <i>string</i> } }</code> </p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteSlotTypeVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteSlotTypeVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteSlotTypeVersionError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return DeleteSlotTypeVersionError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteSlotTypeVersionError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteSlotTypeVersionError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteSlotTypeVersionError::NotFound(String::from(error_message))
                }
                "ResourceInUseException" => {
                    return DeleteSlotTypeVersionError::ResourceInUse(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteSlotTypeVersionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteSlotTypeVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteSlotTypeVersionError {
    fn from(err: serde_json::error::Error) -> DeleteSlotTypeVersionError {
        DeleteSlotTypeVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSlotTypeVersionError {
    fn from(err: CredentialsError) -> DeleteSlotTypeVersionError {
        DeleteSlotTypeVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSlotTypeVersionError {
    fn from(err: HttpDispatchError) -> DeleteSlotTypeVersionError {
        DeleteSlotTypeVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSlotTypeVersionError {
    fn from(err: io::Error) -> DeleteSlotTypeVersionError {
        DeleteSlotTypeVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSlotTypeVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSlotTypeVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteSlotTypeVersionError::BadRequest(ref cause) => cause,
            DeleteSlotTypeVersionError::Conflict(ref cause) => cause,
            DeleteSlotTypeVersionError::InternalFailure(ref cause) => cause,
            DeleteSlotTypeVersionError::LimitExceeded(ref cause) => cause,
            DeleteSlotTypeVersionError::NotFound(ref cause) => cause,
            DeleteSlotTypeVersionError::ResourceInUse(ref cause) => cause,
            DeleteSlotTypeVersionError::Validation(ref cause) => cause,
            DeleteSlotTypeVersionError::Credentials(ref err) => err.description(),
            DeleteSlotTypeVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSlotTypeVersionError::ParseError(ref cause) => cause,
            DeleteSlotTypeVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteUtterances
#[derive(Debug, PartialEq)]
pub enum DeleteUtterancesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteUtterancesError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteUtterancesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteUtterancesError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteUtterancesError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteUtterancesError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteUtterancesError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteUtterancesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteUtterancesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteUtterancesError {
    fn from(err: serde_json::error::Error) -> DeleteUtterancesError {
        DeleteUtterancesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUtterancesError {
    fn from(err: CredentialsError) -> DeleteUtterancesError {
        DeleteUtterancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUtterancesError {
    fn from(err: HttpDispatchError) -> DeleteUtterancesError {
        DeleteUtterancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUtterancesError {
    fn from(err: io::Error) -> DeleteUtterancesError {
        DeleteUtterancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUtterancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUtterancesError {
    fn description(&self) -> &str {
        match *self {
            DeleteUtterancesError::BadRequest(ref cause) => cause,
            DeleteUtterancesError::InternalFailure(ref cause) => cause,
            DeleteUtterancesError::LimitExceeded(ref cause) => cause,
            DeleteUtterancesError::NotFound(ref cause) => cause,
            DeleteUtterancesError::Validation(ref cause) => cause,
            DeleteUtterancesError::Credentials(ref err) => err.description(),
            DeleteUtterancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUtterancesError::ParseError(ref cause) => cause,
            DeleteUtterancesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBot
#[derive(Debug, PartialEq)]
pub enum GetBotError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBotError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return GetBotError::NotFound(String::from(error_message)),
                "ValidationException" => return GetBotError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetBotError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotError {
    fn from(err: serde_json::error::Error) -> GetBotError {
        GetBotError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotError {
    fn from(err: CredentialsError) -> GetBotError {
        GetBotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotError {
    fn from(err: HttpDispatchError) -> GetBotError {
        GetBotError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotError {
    fn from(err: io::Error) -> GetBotError {
        GetBotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotError {
    fn description(&self) -> &str {
        match *self {
            GetBotError::BadRequest(ref cause) => cause,
            GetBotError::InternalFailure(ref cause) => cause,
            GetBotError::LimitExceeded(ref cause) => cause,
            GetBotError::NotFound(ref cause) => cause,
            GetBotError::Validation(ref cause) => cause,
            GetBotError::Credentials(ref err) => err.description(),
            GetBotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBotError::ParseError(ref cause) => cause,
            GetBotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBotAlias
#[derive(Debug, PartialEq)]
pub enum GetBotAliasError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotAliasError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotAliasError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBotAliasError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetBotAliasError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBotAliasError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBotAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotAliasError {
    fn from(err: serde_json::error::Error) -> GetBotAliasError {
        GetBotAliasError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotAliasError {
    fn from(err: CredentialsError) -> GetBotAliasError {
        GetBotAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotAliasError {
    fn from(err: HttpDispatchError) -> GetBotAliasError {
        GetBotAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotAliasError {
    fn from(err: io::Error) -> GetBotAliasError {
        GetBotAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotAliasError {
    fn description(&self) -> &str {
        match *self {
            GetBotAliasError::BadRequest(ref cause) => cause,
            GetBotAliasError::InternalFailure(ref cause) => cause,
            GetBotAliasError::LimitExceeded(ref cause) => cause,
            GetBotAliasError::NotFound(ref cause) => cause,
            GetBotAliasError::Validation(ref cause) => cause,
            GetBotAliasError::Credentials(ref err) => err.description(),
            GetBotAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBotAliasError::ParseError(ref cause) => cause,
            GetBotAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBotAliases
#[derive(Debug, PartialEq)]
pub enum GetBotAliasesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotAliasesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotAliasesError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotAliasesError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBotAliasesError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBotAliasesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBotAliasesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotAliasesError {
    fn from(err: serde_json::error::Error) -> GetBotAliasesError {
        GetBotAliasesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotAliasesError {
    fn from(err: CredentialsError) -> GetBotAliasesError {
        GetBotAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotAliasesError {
    fn from(err: HttpDispatchError) -> GetBotAliasesError {
        GetBotAliasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotAliasesError {
    fn from(err: io::Error) -> GetBotAliasesError {
        GetBotAliasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotAliasesError {
    fn description(&self) -> &str {
        match *self {
            GetBotAliasesError::BadRequest(ref cause) => cause,
            GetBotAliasesError::InternalFailure(ref cause) => cause,
            GetBotAliasesError::LimitExceeded(ref cause) => cause,
            GetBotAliasesError::Validation(ref cause) => cause,
            GetBotAliasesError::Credentials(ref err) => err.description(),
            GetBotAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBotAliasesError::ParseError(ref cause) => cause,
            GetBotAliasesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBotChannelAssociation
#[derive(Debug, PartialEq)]
pub enum GetBotChannelAssociationError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotChannelAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotChannelAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotChannelAssociationError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotChannelAssociationError::InternalFailure(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return GetBotChannelAssociationError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetBotChannelAssociationError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBotChannelAssociationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBotChannelAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotChannelAssociationError {
    fn from(err: serde_json::error::Error) -> GetBotChannelAssociationError {
        GetBotChannelAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotChannelAssociationError {
    fn from(err: CredentialsError) -> GetBotChannelAssociationError {
        GetBotChannelAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotChannelAssociationError {
    fn from(err: HttpDispatchError) -> GetBotChannelAssociationError {
        GetBotChannelAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotChannelAssociationError {
    fn from(err: io::Error) -> GetBotChannelAssociationError {
        GetBotChannelAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotChannelAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotChannelAssociationError {
    fn description(&self) -> &str {
        match *self {
            GetBotChannelAssociationError::BadRequest(ref cause) => cause,
            GetBotChannelAssociationError::InternalFailure(ref cause) => cause,
            GetBotChannelAssociationError::LimitExceeded(ref cause) => cause,
            GetBotChannelAssociationError::NotFound(ref cause) => cause,
            GetBotChannelAssociationError::Validation(ref cause) => cause,
            GetBotChannelAssociationError::Credentials(ref err) => err.description(),
            GetBotChannelAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBotChannelAssociationError::ParseError(ref cause) => cause,
            GetBotChannelAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBotChannelAssociations
#[derive(Debug, PartialEq)]
pub enum GetBotChannelAssociationsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotChannelAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotChannelAssociationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotChannelAssociationsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotChannelAssociationsError::InternalFailure(String::from(
                        error_message,
                    ))
                }
                "LimitExceededException" => {
                    return GetBotChannelAssociationsError::LimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetBotChannelAssociationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBotChannelAssociationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotChannelAssociationsError {
    fn from(err: serde_json::error::Error) -> GetBotChannelAssociationsError {
        GetBotChannelAssociationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotChannelAssociationsError {
    fn from(err: CredentialsError) -> GetBotChannelAssociationsError {
        GetBotChannelAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotChannelAssociationsError {
    fn from(err: HttpDispatchError) -> GetBotChannelAssociationsError {
        GetBotChannelAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotChannelAssociationsError {
    fn from(err: io::Error) -> GetBotChannelAssociationsError {
        GetBotChannelAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotChannelAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotChannelAssociationsError {
    fn description(&self) -> &str {
        match *self {
            GetBotChannelAssociationsError::BadRequest(ref cause) => cause,
            GetBotChannelAssociationsError::InternalFailure(ref cause) => cause,
            GetBotChannelAssociationsError::LimitExceeded(ref cause) => cause,
            GetBotChannelAssociationsError::Validation(ref cause) => cause,
            GetBotChannelAssociationsError::Credentials(ref err) => err.description(),
            GetBotChannelAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBotChannelAssociationsError::ParseError(ref cause) => cause,
            GetBotChannelAssociationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBotVersions
#[derive(Debug, PartialEq)]
pub enum GetBotVersionsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotVersionsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotVersionsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBotVersionsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetBotVersionsError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBotVersionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBotVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotVersionsError {
    fn from(err: serde_json::error::Error) -> GetBotVersionsError {
        GetBotVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotVersionsError {
    fn from(err: CredentialsError) -> GetBotVersionsError {
        GetBotVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotVersionsError {
    fn from(err: HttpDispatchError) -> GetBotVersionsError {
        GetBotVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotVersionsError {
    fn from(err: io::Error) -> GetBotVersionsError {
        GetBotVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetBotVersionsError::BadRequest(ref cause) => cause,
            GetBotVersionsError::InternalFailure(ref cause) => cause,
            GetBotVersionsError::LimitExceeded(ref cause) => cause,
            GetBotVersionsError::NotFound(ref cause) => cause,
            GetBotVersionsError::Validation(ref cause) => cause,
            GetBotVersionsError::Credentials(ref err) => err.description(),
            GetBotVersionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBotVersionsError::ParseError(ref cause) => cause,
            GetBotVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBots
#[derive(Debug, PartialEq)]
pub enum GetBotsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBotsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBotsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBotsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBotsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBotsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return GetBotsError::NotFound(String::from(error_message)),
                "ValidationException" => return GetBotsError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetBotsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBotsError {
    fn from(err: serde_json::error::Error) -> GetBotsError {
        GetBotsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBotsError {
    fn from(err: CredentialsError) -> GetBotsError {
        GetBotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBotsError {
    fn from(err: HttpDispatchError) -> GetBotsError {
        GetBotsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBotsError {
    fn from(err: io::Error) -> GetBotsError {
        GetBotsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBotsError {
    fn description(&self) -> &str {
        match *self {
            GetBotsError::BadRequest(ref cause) => cause,
            GetBotsError::InternalFailure(ref cause) => cause,
            GetBotsError::LimitExceeded(ref cause) => cause,
            GetBotsError::NotFound(ref cause) => cause,
            GetBotsError::Validation(ref cause) => cause,
            GetBotsError::Credentials(ref err) => err.description(),
            GetBotsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBotsError::ParseError(ref cause) => cause,
            GetBotsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBuiltinIntent
#[derive(Debug, PartialEq)]
pub enum GetBuiltinIntentError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBuiltinIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBuiltinIntentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBuiltinIntentError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBuiltinIntentError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBuiltinIntentError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetBuiltinIntentError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBuiltinIntentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBuiltinIntentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBuiltinIntentError {
    fn from(err: serde_json::error::Error) -> GetBuiltinIntentError {
        GetBuiltinIntentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBuiltinIntentError {
    fn from(err: CredentialsError) -> GetBuiltinIntentError {
        GetBuiltinIntentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBuiltinIntentError {
    fn from(err: HttpDispatchError) -> GetBuiltinIntentError {
        GetBuiltinIntentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBuiltinIntentError {
    fn from(err: io::Error) -> GetBuiltinIntentError {
        GetBuiltinIntentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBuiltinIntentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBuiltinIntentError {
    fn description(&self) -> &str {
        match *self {
            GetBuiltinIntentError::BadRequest(ref cause) => cause,
            GetBuiltinIntentError::InternalFailure(ref cause) => cause,
            GetBuiltinIntentError::LimitExceeded(ref cause) => cause,
            GetBuiltinIntentError::NotFound(ref cause) => cause,
            GetBuiltinIntentError::Validation(ref cause) => cause,
            GetBuiltinIntentError::Credentials(ref err) => err.description(),
            GetBuiltinIntentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetBuiltinIntentError::ParseError(ref cause) => cause,
            GetBuiltinIntentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBuiltinIntents
#[derive(Debug, PartialEq)]
pub enum GetBuiltinIntentsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBuiltinIntentsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBuiltinIntentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBuiltinIntentsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBuiltinIntentsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBuiltinIntentsError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBuiltinIntentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBuiltinIntentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBuiltinIntentsError {
    fn from(err: serde_json::error::Error) -> GetBuiltinIntentsError {
        GetBuiltinIntentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBuiltinIntentsError {
    fn from(err: CredentialsError) -> GetBuiltinIntentsError {
        GetBuiltinIntentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBuiltinIntentsError {
    fn from(err: HttpDispatchError) -> GetBuiltinIntentsError {
        GetBuiltinIntentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBuiltinIntentsError {
    fn from(err: io::Error) -> GetBuiltinIntentsError {
        GetBuiltinIntentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBuiltinIntentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBuiltinIntentsError {
    fn description(&self) -> &str {
        match *self {
            GetBuiltinIntentsError::BadRequest(ref cause) => cause,
            GetBuiltinIntentsError::InternalFailure(ref cause) => cause,
            GetBuiltinIntentsError::LimitExceeded(ref cause) => cause,
            GetBuiltinIntentsError::Validation(ref cause) => cause,
            GetBuiltinIntentsError::Credentials(ref err) => err.description(),
            GetBuiltinIntentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBuiltinIntentsError::ParseError(ref cause) => cause,
            GetBuiltinIntentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBuiltinSlotTypes
#[derive(Debug, PartialEq)]
pub enum GetBuiltinSlotTypesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetBuiltinSlotTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetBuiltinSlotTypesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetBuiltinSlotTypesError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetBuiltinSlotTypesError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetBuiltinSlotTypesError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBuiltinSlotTypesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBuiltinSlotTypesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBuiltinSlotTypesError {
    fn from(err: serde_json::error::Error) -> GetBuiltinSlotTypesError {
        GetBuiltinSlotTypesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBuiltinSlotTypesError {
    fn from(err: CredentialsError) -> GetBuiltinSlotTypesError {
        GetBuiltinSlotTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBuiltinSlotTypesError {
    fn from(err: HttpDispatchError) -> GetBuiltinSlotTypesError {
        GetBuiltinSlotTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBuiltinSlotTypesError {
    fn from(err: io::Error) -> GetBuiltinSlotTypesError {
        GetBuiltinSlotTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBuiltinSlotTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBuiltinSlotTypesError {
    fn description(&self) -> &str {
        match *self {
            GetBuiltinSlotTypesError::BadRequest(ref cause) => cause,
            GetBuiltinSlotTypesError::InternalFailure(ref cause) => cause,
            GetBuiltinSlotTypesError::LimitExceeded(ref cause) => cause,
            GetBuiltinSlotTypesError::Validation(ref cause) => cause,
            GetBuiltinSlotTypesError::Credentials(ref err) => err.description(),
            GetBuiltinSlotTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBuiltinSlotTypesError::ParseError(ref cause) => cause,
            GetBuiltinSlotTypesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetExport
#[derive(Debug, PartialEq)]
pub enum GetExportError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetExportError {
    pub fn from_response(res: BufferedHttpResponse) -> GetExportError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetExportError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetExportError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetExportError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return GetExportError::NotFound(String::from(error_message)),
                "ValidationException" => {
                    return GetExportError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetExportError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetExportError {
    fn from(err: serde_json::error::Error) -> GetExportError {
        GetExportError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetExportError {
    fn from(err: CredentialsError) -> GetExportError {
        GetExportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetExportError {
    fn from(err: HttpDispatchError) -> GetExportError {
        GetExportError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetExportError {
    fn from(err: io::Error) -> GetExportError {
        GetExportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetExportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportError {
    fn description(&self) -> &str {
        match *self {
            GetExportError::BadRequest(ref cause) => cause,
            GetExportError::InternalFailure(ref cause) => cause,
            GetExportError::LimitExceeded(ref cause) => cause,
            GetExportError::NotFound(ref cause) => cause,
            GetExportError::Validation(ref cause) => cause,
            GetExportError::Credentials(ref err) => err.description(),
            GetExportError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetExportError::ParseError(ref cause) => cause,
            GetExportError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetImport
#[derive(Debug, PartialEq)]
pub enum GetImportError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetImportError {
    pub fn from_response(res: BufferedHttpResponse) -> GetImportError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetImportError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetImportError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetImportError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return GetImportError::NotFound(String::from(error_message)),
                "ValidationException" => {
                    return GetImportError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetImportError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetImportError {
    fn from(err: serde_json::error::Error) -> GetImportError {
        GetImportError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetImportError {
    fn from(err: CredentialsError) -> GetImportError {
        GetImportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetImportError {
    fn from(err: HttpDispatchError) -> GetImportError {
        GetImportError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetImportError {
    fn from(err: io::Error) -> GetImportError {
        GetImportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportError {
    fn description(&self) -> &str {
        match *self {
            GetImportError::BadRequest(ref cause) => cause,
            GetImportError::InternalFailure(ref cause) => cause,
            GetImportError::LimitExceeded(ref cause) => cause,
            GetImportError::NotFound(ref cause) => cause,
            GetImportError::Validation(ref cause) => cause,
            GetImportError::Credentials(ref err) => err.description(),
            GetImportError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetImportError::ParseError(ref cause) => cause,
            GetImportError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetIntent
#[derive(Debug, PartialEq)]
pub enum GetIntentError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> GetIntentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetIntentError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetIntentError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetIntentError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => return GetIntentError::NotFound(String::from(error_message)),
                "ValidationException" => {
                    return GetIntentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetIntentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetIntentError {
    fn from(err: serde_json::error::Error) -> GetIntentError {
        GetIntentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIntentError {
    fn from(err: CredentialsError) -> GetIntentError {
        GetIntentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIntentError {
    fn from(err: HttpDispatchError) -> GetIntentError {
        GetIntentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIntentError {
    fn from(err: io::Error) -> GetIntentError {
        GetIntentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIntentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntentError {
    fn description(&self) -> &str {
        match *self {
            GetIntentError::BadRequest(ref cause) => cause,
            GetIntentError::InternalFailure(ref cause) => cause,
            GetIntentError::LimitExceeded(ref cause) => cause,
            GetIntentError::NotFound(ref cause) => cause,
            GetIntentError::Validation(ref cause) => cause,
            GetIntentError::Credentials(ref err) => err.description(),
            GetIntentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetIntentError::ParseError(ref cause) => cause,
            GetIntentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetIntentVersions
#[derive(Debug, PartialEq)]
pub enum GetIntentVersionsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetIntentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetIntentVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetIntentVersionsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetIntentVersionsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetIntentVersionsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetIntentVersionsError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetIntentVersionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetIntentVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetIntentVersionsError {
    fn from(err: serde_json::error::Error) -> GetIntentVersionsError {
        GetIntentVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIntentVersionsError {
    fn from(err: CredentialsError) -> GetIntentVersionsError {
        GetIntentVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIntentVersionsError {
    fn from(err: HttpDispatchError) -> GetIntentVersionsError {
        GetIntentVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIntentVersionsError {
    fn from(err: io::Error) -> GetIntentVersionsError {
        GetIntentVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIntentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntentVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetIntentVersionsError::BadRequest(ref cause) => cause,
            GetIntentVersionsError::InternalFailure(ref cause) => cause,
            GetIntentVersionsError::LimitExceeded(ref cause) => cause,
            GetIntentVersionsError::NotFound(ref cause) => cause,
            GetIntentVersionsError::Validation(ref cause) => cause,
            GetIntentVersionsError::Credentials(ref err) => err.description(),
            GetIntentVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIntentVersionsError::ParseError(ref cause) => cause,
            GetIntentVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetIntents
#[derive(Debug, PartialEq)]
pub enum GetIntentsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetIntentsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetIntentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetIntentsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetIntentsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetIntentsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetIntentsError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetIntentsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetIntentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetIntentsError {
    fn from(err: serde_json::error::Error) -> GetIntentsError {
        GetIntentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIntentsError {
    fn from(err: CredentialsError) -> GetIntentsError {
        GetIntentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIntentsError {
    fn from(err: HttpDispatchError) -> GetIntentsError {
        GetIntentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIntentsError {
    fn from(err: io::Error) -> GetIntentsError {
        GetIntentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIntentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntentsError {
    fn description(&self) -> &str {
        match *self {
            GetIntentsError::BadRequest(ref cause) => cause,
            GetIntentsError::InternalFailure(ref cause) => cause,
            GetIntentsError::LimitExceeded(ref cause) => cause,
            GetIntentsError::NotFound(ref cause) => cause,
            GetIntentsError::Validation(ref cause) => cause,
            GetIntentsError::Credentials(ref err) => err.description(),
            GetIntentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetIntentsError::ParseError(ref cause) => cause,
            GetIntentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSlotType
#[derive(Debug, PartialEq)]
pub enum GetSlotTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> GetSlotTypeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetSlotTypeError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetSlotTypeError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetSlotTypeError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSlotTypeError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSlotTypeError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSlotTypeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSlotTypeError {
    fn from(err: serde_json::error::Error) -> GetSlotTypeError {
        GetSlotTypeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSlotTypeError {
    fn from(err: CredentialsError) -> GetSlotTypeError {
        GetSlotTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSlotTypeError {
    fn from(err: HttpDispatchError) -> GetSlotTypeError {
        GetSlotTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSlotTypeError {
    fn from(err: io::Error) -> GetSlotTypeError {
        GetSlotTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSlotTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSlotTypeError {
    fn description(&self) -> &str {
        match *self {
            GetSlotTypeError::BadRequest(ref cause) => cause,
            GetSlotTypeError::InternalFailure(ref cause) => cause,
            GetSlotTypeError::LimitExceeded(ref cause) => cause,
            GetSlotTypeError::NotFound(ref cause) => cause,
            GetSlotTypeError::Validation(ref cause) => cause,
            GetSlotTypeError::Credentials(ref err) => err.description(),
            GetSlotTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSlotTypeError::ParseError(ref cause) => cause,
            GetSlotTypeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSlotTypeVersions
#[derive(Debug, PartialEq)]
pub enum GetSlotTypeVersionsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSlotTypeVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetSlotTypeVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetSlotTypeVersionsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetSlotTypeVersionsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetSlotTypeVersionsError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSlotTypeVersionsError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSlotTypeVersionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSlotTypeVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSlotTypeVersionsError {
    fn from(err: serde_json::error::Error) -> GetSlotTypeVersionsError {
        GetSlotTypeVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSlotTypeVersionsError {
    fn from(err: CredentialsError) -> GetSlotTypeVersionsError {
        GetSlotTypeVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSlotTypeVersionsError {
    fn from(err: HttpDispatchError) -> GetSlotTypeVersionsError {
        GetSlotTypeVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSlotTypeVersionsError {
    fn from(err: io::Error) -> GetSlotTypeVersionsError {
        GetSlotTypeVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSlotTypeVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSlotTypeVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetSlotTypeVersionsError::BadRequest(ref cause) => cause,
            GetSlotTypeVersionsError::InternalFailure(ref cause) => cause,
            GetSlotTypeVersionsError::LimitExceeded(ref cause) => cause,
            GetSlotTypeVersionsError::NotFound(ref cause) => cause,
            GetSlotTypeVersionsError::Validation(ref cause) => cause,
            GetSlotTypeVersionsError::Credentials(ref err) => err.description(),
            GetSlotTypeVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSlotTypeVersionsError::ParseError(ref cause) => cause,
            GetSlotTypeVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetSlotTypes
#[derive(Debug, PartialEq)]
pub enum GetSlotTypesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource and try again.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetSlotTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> GetSlotTypesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetSlotTypesError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetSlotTypesError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetSlotTypesError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetSlotTypesError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetSlotTypesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetSlotTypesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetSlotTypesError {
    fn from(err: serde_json::error::Error) -> GetSlotTypesError {
        GetSlotTypesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSlotTypesError {
    fn from(err: CredentialsError) -> GetSlotTypesError {
        GetSlotTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSlotTypesError {
    fn from(err: HttpDispatchError) -> GetSlotTypesError {
        GetSlotTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSlotTypesError {
    fn from(err: io::Error) -> GetSlotTypesError {
        GetSlotTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSlotTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSlotTypesError {
    fn description(&self) -> &str {
        match *self {
            GetSlotTypesError::BadRequest(ref cause) => cause,
            GetSlotTypesError::InternalFailure(ref cause) => cause,
            GetSlotTypesError::LimitExceeded(ref cause) => cause,
            GetSlotTypesError::NotFound(ref cause) => cause,
            GetSlotTypesError::Validation(ref cause) => cause,
            GetSlotTypesError::Credentials(ref err) => err.description(),
            GetSlotTypesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSlotTypesError::ParseError(ref cause) => cause,
            GetSlotTypesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetUtterancesView
#[derive(Debug, PartialEq)]
pub enum GetUtterancesViewError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetUtterancesViewError {
    pub fn from_response(res: BufferedHttpResponse) -> GetUtterancesViewError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetUtterancesViewError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetUtterancesViewError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetUtterancesViewError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return GetUtterancesViewError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetUtterancesViewError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetUtterancesViewError {
    fn from(err: serde_json::error::Error) -> GetUtterancesViewError {
        GetUtterancesViewError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUtterancesViewError {
    fn from(err: CredentialsError) -> GetUtterancesViewError {
        GetUtterancesViewError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUtterancesViewError {
    fn from(err: HttpDispatchError) -> GetUtterancesViewError {
        GetUtterancesViewError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUtterancesViewError {
    fn from(err: io::Error) -> GetUtterancesViewError {
        GetUtterancesViewError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUtterancesViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUtterancesViewError {
    fn description(&self) -> &str {
        match *self {
            GetUtterancesViewError::BadRequest(ref cause) => cause,
            GetUtterancesViewError::InternalFailure(ref cause) => cause,
            GetUtterancesViewError::LimitExceeded(ref cause) => cause,
            GetUtterancesViewError::Validation(ref cause) => cause,
            GetUtterancesViewError::Credentials(ref err) => err.description(),
            GetUtterancesViewError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetUtterancesViewError::ParseError(ref cause) => cause,
            GetUtterancesViewError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutBot
#[derive(Debug, PartialEq)]
pub enum PutBotError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutBotError {
    pub fn from_response(res: BufferedHttpResponse) -> PutBotError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return PutBotError::BadRequest(String::from(error_message))
                }
                "ConflictException" => return PutBotError::Conflict(String::from(error_message)),
                "InternalFailureException" => {
                    return PutBotError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PutBotError::LimitExceeded(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return PutBotError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => return PutBotError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return PutBotError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutBotError {
    fn from(err: serde_json::error::Error) -> PutBotError {
        PutBotError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutBotError {
    fn from(err: CredentialsError) -> PutBotError {
        PutBotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBotError {
    fn from(err: HttpDispatchError) -> PutBotError {
        PutBotError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBotError {
    fn from(err: io::Error) -> PutBotError {
        PutBotError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBotError {
    fn description(&self) -> &str {
        match *self {
            PutBotError::BadRequest(ref cause) => cause,
            PutBotError::Conflict(ref cause) => cause,
            PutBotError::InternalFailure(ref cause) => cause,
            PutBotError::LimitExceeded(ref cause) => cause,
            PutBotError::PreconditionFailed(ref cause) => cause,
            PutBotError::Validation(ref cause) => cause,
            PutBotError::Credentials(ref err) => err.description(),
            PutBotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBotError::ParseError(ref cause) => cause,
            PutBotError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutBotAlias
#[derive(Debug, PartialEq)]
pub enum PutBotAliasError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> PutBotAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return PutBotAliasError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return PutBotAliasError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return PutBotAliasError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PutBotAliasError::LimitExceeded(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return PutBotAliasError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return PutBotAliasError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutBotAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutBotAliasError {
    fn from(err: serde_json::error::Error) -> PutBotAliasError {
        PutBotAliasError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutBotAliasError {
    fn from(err: CredentialsError) -> PutBotAliasError {
        PutBotAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutBotAliasError {
    fn from(err: HttpDispatchError) -> PutBotAliasError {
        PutBotAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutBotAliasError {
    fn from(err: io::Error) -> PutBotAliasError {
        PutBotAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutBotAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutBotAliasError {
    fn description(&self) -> &str {
        match *self {
            PutBotAliasError::BadRequest(ref cause) => cause,
            PutBotAliasError::Conflict(ref cause) => cause,
            PutBotAliasError::InternalFailure(ref cause) => cause,
            PutBotAliasError::LimitExceeded(ref cause) => cause,
            PutBotAliasError::PreconditionFailed(ref cause) => cause,
            PutBotAliasError::Validation(ref cause) => cause,
            PutBotAliasError::Credentials(ref err) => err.description(),
            PutBotAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutBotAliasError::ParseError(ref cause) => cause,
            PutBotAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutIntent
#[derive(Debug, PartialEq)]
pub enum PutIntentError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> PutIntentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return PutIntentError::BadRequest(String::from(error_message))
                }
                "ConflictException" => return PutIntentError::Conflict(String::from(error_message)),
                "InternalFailureException" => {
                    return PutIntentError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PutIntentError::LimitExceeded(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return PutIntentError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return PutIntentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutIntentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutIntentError {
    fn from(err: serde_json::error::Error) -> PutIntentError {
        PutIntentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutIntentError {
    fn from(err: CredentialsError) -> PutIntentError {
        PutIntentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutIntentError {
    fn from(err: HttpDispatchError) -> PutIntentError {
        PutIntentError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutIntentError {
    fn from(err: io::Error) -> PutIntentError {
        PutIntentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutIntentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIntentError {
    fn description(&self) -> &str {
        match *self {
            PutIntentError::BadRequest(ref cause) => cause,
            PutIntentError::Conflict(ref cause) => cause,
            PutIntentError::InternalFailure(ref cause) => cause,
            PutIntentError::LimitExceeded(ref cause) => cause,
            PutIntentError::PreconditionFailed(ref cause) => cause,
            PutIntentError::Validation(ref cause) => cause,
            PutIntentError::Credentials(ref err) => err.description(),
            PutIntentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutIntentError::ParseError(ref cause) => cause,
            PutIntentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutSlotType
#[derive(Debug, PartialEq)]
pub enum PutSlotTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p> There was a conflict processing the request. Try your request again. </p>
    Conflict(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p> The checksum of the resource that you are trying to change does not match the checksum in the request. Check the resource's checksum and try again.</p>
    PreconditionFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> PutSlotTypeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return PutSlotTypeError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return PutSlotTypeError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return PutSlotTypeError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return PutSlotTypeError::LimitExceeded(String::from(error_message))
                }
                "PreconditionFailedException" => {
                    return PutSlotTypeError::PreconditionFailed(String::from(error_message))
                }
                "ValidationException" => {
                    return PutSlotTypeError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return PutSlotTypeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutSlotTypeError {
    fn from(err: serde_json::error::Error) -> PutSlotTypeError {
        PutSlotTypeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSlotTypeError {
    fn from(err: CredentialsError) -> PutSlotTypeError {
        PutSlotTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSlotTypeError {
    fn from(err: HttpDispatchError) -> PutSlotTypeError {
        PutSlotTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSlotTypeError {
    fn from(err: io::Error) -> PutSlotTypeError {
        PutSlotTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSlotTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSlotTypeError {
    fn description(&self) -> &str {
        match *self {
            PutSlotTypeError::BadRequest(ref cause) => cause,
            PutSlotTypeError::Conflict(ref cause) => cause,
            PutSlotTypeError::InternalFailure(ref cause) => cause,
            PutSlotTypeError::LimitExceeded(ref cause) => cause,
            PutSlotTypeError::PreconditionFailed(ref cause) => cause,
            PutSlotTypeError::Validation(ref cause) => cause,
            PutSlotTypeError::Credentials(ref err) => err.description(),
            PutSlotTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutSlotTypeError::ParseError(ref cause) => cause,
            PutSlotTypeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartImport
#[derive(Debug, PartialEq)]
pub enum StartImportError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl StartImportError {
    pub fn from_response(res: BufferedHttpResponse) -> StartImportError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return StartImportError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return StartImportError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartImportError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return StartImportError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartImportError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartImportError {
    fn from(err: serde_json::error::Error) -> StartImportError {
        StartImportError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartImportError {
    fn from(err: CredentialsError) -> StartImportError {
        StartImportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartImportError {
    fn from(err: HttpDispatchError) -> StartImportError {
        StartImportError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartImportError {
    fn from(err: io::Error) -> StartImportError {
        StartImportError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartImportError {
    fn description(&self) -> &str {
        match *self {
            StartImportError::BadRequest(ref cause) => cause,
            StartImportError::InternalFailure(ref cause) => cause,
            StartImportError::LimitExceeded(ref cause) => cause,
            StartImportError::Validation(ref cause) => cause,
            StartImportError::Credentials(ref err) => err.description(),
            StartImportError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartImportError::ParseError(ref cause) => cause,
            StartImportError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Lex Model Building Service API. Amazon Lex Model Building Service clients implement this trait.
pub trait LexModels {
    /// <p>Creates a new version of the bot based on the <code>$LATEST</code> version. If the <code>$LATEST</code> version of this resource hasn't changed since you created the last version, Amazon Lex doesn't create a new version. It returns the last created version.</p> <note> <p>You can update only the <code>$LATEST</code> version of the bot. You can't update the numbered versions that you create with the <code>CreateBotVersion</code> operation.</p> </note> <p> When you create the first version of a bot, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p> This operation requires permission for the <code>lex:CreateBotVersion</code> action. </p>
    fn create_bot_version(
        &self,
        input: CreateBotVersionRequest,
    ) -> RusotoFuture<CreateBotVersionResponse, CreateBotVersionError>;

    /// <p>Creates a new version of an intent based on the <code>$LATEST</code> version of the intent. If the <code>$LATEST</code> version of this intent hasn't changed since you last updated it, Amazon Lex doesn't create a new version. It returns the last version you created.</p> <note> <p>You can update only the <code>$LATEST</code> version of the intent. You can't update the numbered versions that you create with the <code>CreateIntentVersion</code> operation.</p> </note> <p> When you create a version of an intent, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions to perform the <code>lex:CreateIntentVersion</code> action. </p>
    fn create_intent_version(
        &self,
        input: CreateIntentVersionRequest,
    ) -> RusotoFuture<CreateIntentVersionResponse, CreateIntentVersionError>;

    /// <p>Creates a new version of a slot type based on the <code>$LATEST</code> version of the specified slot type. If the <code>$LATEST</code> version of this resource has not changed since the last version that you created, Amazon Lex doesn't create a new version. It returns the last version that you created. </p> <note> <p>You can update only the <code>$LATEST</code> version of a slot type. You can't update the numbered versions that you create with the <code>CreateSlotTypeVersion</code> operation.</p> </note> <p>When you create a version of a slot type, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions for the <code>lex:CreateSlotTypeVersion</code> action.</p>
    fn create_slot_type_version(
        &self,
        input: CreateSlotTypeVersionRequest,
    ) -> RusotoFuture<CreateSlotTypeVersionResponse, CreateSlotTypeVersionError>;

    /// <p>Deletes all versions of the bot, including the <code>$LATEST</code> version. To delete a specific version of the bot, use the <a>DeleteBotVersion</a> operation.</p> <p>If a bot has an alias, you can't delete it. Instead, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the alias that refers to the bot. To remove the reference to the bot, delete the alias. If you get the same exception again, delete the referring alias until the <code>DeleteBot</code> operation is successful.</p> <p>This operation requires permissions for the <code>lex:DeleteBot</code> action.</p>
    fn delete_bot(&self, input: DeleteBotRequest) -> RusotoFuture<(), DeleteBotError>;

    /// <p>Deletes an alias for the specified bot. </p> <p>You can't delete an alias that is used in the association between a bot and a messaging channel. If an alias is used in a channel association, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the channel association that refers to the bot. You can remove the reference to the alias by deleting the channel association. If you get the same exception again, delete the referring association until the <code>DeleteBotAlias</code> operation is successful.</p>
    fn delete_bot_alias(
        &self,
        input: DeleteBotAliasRequest,
    ) -> RusotoFuture<(), DeleteBotAliasError>;

    /// <p>Deletes the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permission for the <code>lex:DeleteBotChannelAssociation</code> action.</p>
    fn delete_bot_channel_association(
        &self,
        input: DeleteBotChannelAssociationRequest,
    ) -> RusotoFuture<(), DeleteBotChannelAssociationError>;

    /// <p>Deletes a specific version of a bot. To delete all versions of a bot, use the <a>DeleteBot</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteBotVersion</code> action.</p>
    fn delete_bot_version(
        &self,
        input: DeleteBotVersionRequest,
    ) -> RusotoFuture<(), DeleteBotVersionError>;

    /// <p>Deletes all versions of the intent, including the <code>$LATEST</code> version. To delete a specific version of the intent, use the <a>DeleteIntentVersion</a> operation.</p> <p> You can delete a version of an intent only if it is not referenced. To delete an intent that is referred to in one or more bots (see <a>how-it-works</a>), you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, it provides an example reference that shows where the intent is referenced. To remove the reference to the intent, either update the bot or delete it. If you get the same exception when you attempt to delete the intent again, repeat until the intent has no references and the call to <code>DeleteIntent</code> is successful. </p> </note> <p> This operation requires permission for the <code>lex:DeleteIntent</code> action. </p>
    fn delete_intent(&self, input: DeleteIntentRequest) -> RusotoFuture<(), DeleteIntentError>;

    /// <p>Deletes a specific version of an intent. To delete all versions of a intent, use the <a>DeleteIntent</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteIntentVersion</code> action.</p>
    fn delete_intent_version(
        &self,
        input: DeleteIntentVersionRequest,
    ) -> RusotoFuture<(), DeleteIntentVersionError>;

    /// <p>Deletes all versions of the slot type, including the <code>$LATEST</code> version. To delete a specific version of the slot type, use the <a>DeleteSlotTypeVersion</a> operation.</p> <p> You can delete a version of a slot type only if it is not referenced. To delete a slot type that is referred to in one or more intents, you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, the exception provides an example reference that shows the intent where the slot type is referenced. To remove the reference to the slot type, either update the intent or delete it. If you get the same exception when you attempt to delete the slot type again, repeat until the slot type has no references and the <code>DeleteSlotType</code> call is successful. </p> </note> <p>This operation requires permission for the <code>lex:DeleteSlotType</code> action.</p>
    fn delete_slot_type(
        &self,
        input: DeleteSlotTypeRequest,
    ) -> RusotoFuture<(), DeleteSlotTypeError>;

    /// <p>Deletes a specific version of a slot type. To delete all versions of a slot type, use the <a>DeleteSlotType</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteSlotTypeVersion</code> action.</p>
    fn delete_slot_type_version(
        &self,
        input: DeleteSlotTypeVersionRequest,
    ) -> RusotoFuture<(), DeleteSlotTypeVersionError>;

    /// <p>Deletes stored utterances.</p> <p>Amazon Lex stores the utterances that users send to your bot. Utterances are stored for 15 days for use with the <a>GetUtterancesView</a> operation, and then stored indefinitely for use in improving the ability of your bot to respond to user input.</p> <p>Use the <code>DeleteStoredUtterances</code> operation to manually delete stored utterances for a specific user.</p> <p>This operation requires permissions for the <code>lex:DeleteUtterances</code> action.</p>
    fn delete_utterances(
        &self,
        input: DeleteUtterancesRequest,
    ) -> RusotoFuture<(), DeleteUtterancesError>;

    /// <p>Returns metadata information for a specific bot. You must provide the bot name and the bot version or alias. </p> <p> This operation requires permissions for the <code>lex:GetBot</code> action. </p>
    fn get_bot(&self, input: GetBotRequest) -> RusotoFuture<GetBotResponse, GetBotError>;

    /// <p>Returns information about an Amazon Lex bot alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:GetBotAlias</code> action.</p>
    fn get_bot_alias(
        &self,
        input: GetBotAliasRequest,
    ) -> RusotoFuture<GetBotAliasResponse, GetBotAliasError>;

    /// <p>Returns a list of aliases for a specified Amazon Lex bot.</p> <p>This operation requires permissions for the <code>lex:GetBotAliases</code> action.</p>
    fn get_bot_aliases(
        &self,
        input: GetBotAliasesRequest,
    ) -> RusotoFuture<GetBotAliasesResponse, GetBotAliasesError>;

    /// <p>Returns information about the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permissions for the <code>lex:GetBotChannelAssociation</code> action.</p>
    fn get_bot_channel_association(
        &self,
        input: GetBotChannelAssociationRequest,
    ) -> RusotoFuture<GetBotChannelAssociationResponse, GetBotChannelAssociationError>;

    /// <p> Returns a list of all of the channels associated with the specified bot. </p> <p>The <code>GetBotChannelAssociations</code> operation requires permissions for the <code>lex:GetBotChannelAssociations</code> action.</p>
    fn get_bot_channel_associations(
        &self,
        input: GetBotChannelAssociationsRequest,
    ) -> RusotoFuture<GetBotChannelAssociationsResponse, GetBotChannelAssociationsError>;

    /// <p>Gets information about all of the versions of a bot.</p> <p>The <code>GetBotVersions</code> operation returns a <code>BotMetadata</code> object for each version of a bot. For example, if a bot has three numbered versions, the <code>GetBotVersions</code> operation returns four <code>BotMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetBotVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetBotVersions</code> action.</p>
    fn get_bot_versions(
        &self,
        input: GetBotVersionsRequest,
    ) -> RusotoFuture<GetBotVersionsResponse, GetBotVersionsError>;

    /// <p>Returns bot information as follows: </p> <ul> <li> <p>If you provide the <code>nameContains</code> field, the response includes information for the <code>$LATEST</code> version of all bots whose name contains the specified string.</p> </li> <li> <p>If you don't specify the <code>nameContains</code> field, the operation returns information about the <code>$LATEST</code> version of all of your bots.</p> </li> </ul> <p>This operation requires permission for the <code>lex:GetBots</code> action.</p>
    fn get_bots(&self, input: GetBotsRequest) -> RusotoFuture<GetBotsResponse, GetBotsError>;

    /// <p>Returns information about a built-in intent.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntent</code> action.</p>
    fn get_builtin_intent(
        &self,
        input: GetBuiltinIntentRequest,
    ) -> RusotoFuture<GetBuiltinIntentResponse, GetBuiltinIntentError>;

    /// <p>Gets a list of built-in intents that meet the specified criteria.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntents</code> action.</p>
    fn get_builtin_intents(
        &self,
        input: GetBuiltinIntentsRequest,
    ) -> RusotoFuture<GetBuiltinIntentsResponse, GetBuiltinIntentsError>;

    /// <p>Gets a list of built-in slot types that meet the specified criteria.</p> <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p> <p>This operation requires permission for the <code>lex:GetBuiltInSlotTypes</code> action.</p>
    fn get_builtin_slot_types(
        &self,
        input: GetBuiltinSlotTypesRequest,
    ) -> RusotoFuture<GetBuiltinSlotTypesResponse, GetBuiltinSlotTypesError>;

    /// <p>Exports the contents of a Amazon Lex resource in a specified format. </p>
    fn get_export(
        &self,
        input: GetExportRequest,
    ) -> RusotoFuture<GetExportResponse, GetExportError>;

    /// <p>Gets information about an import job started with the <code>StartImport</code> operation.</p>
    fn get_import(
        &self,
        input: GetImportRequest,
    ) -> RusotoFuture<GetImportResponse, GetImportError>;

    /// <p> Returns information about an intent. In addition to the intent name, you must specify the intent version. </p> <p> This operation requires permissions to perform the <code>lex:GetIntent</code> action. </p>
    fn get_intent(
        &self,
        input: GetIntentRequest,
    ) -> RusotoFuture<GetIntentResponse, GetIntentError>;

    /// <p>Gets information about all of the versions of an intent.</p> <p>The <code>GetIntentVersions</code> operation returns an <code>IntentMetadata</code> object for each version of an intent. For example, if an intent has three numbered versions, the <code>GetIntentVersions</code> operation returns four <code>IntentMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetIntentVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetIntentVersions</code> action.</p>
    fn get_intent_versions(
        &self,
        input: GetIntentVersionsRequest,
    ) -> RusotoFuture<GetIntentVersionsResponse, GetIntentVersionsError>;

    /// <p>Returns intent information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all intents that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all intents. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetIntents</code> action. </p>
    fn get_intents(
        &self,
        input: GetIntentsRequest,
    ) -> RusotoFuture<GetIntentsResponse, GetIntentsError>;

    /// <p>Returns information about a specific version of a slot type. In addition to specifying the slot type name, you must specify the slot type version.</p> <p>This operation requires permissions for the <code>lex:GetSlotType</code> action.</p>
    fn get_slot_type(
        &self,
        input: GetSlotTypeRequest,
    ) -> RusotoFuture<GetSlotTypeResponse, GetSlotTypeError>;

    /// <p>Gets information about all versions of a slot type.</p> <p>The <code>GetSlotTypeVersions</code> operation returns a <code>SlotTypeMetadata</code> object for each version of a slot type. For example, if a slot type has three numbered versions, the <code>GetSlotTypeVersions</code> operation returns four <code>SlotTypeMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetSlotTypeVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetSlotTypeVersions</code> action.</p>
    fn get_slot_type_versions(
        &self,
        input: GetSlotTypeVersionsRequest,
    ) -> RusotoFuture<GetSlotTypeVersionsResponse, GetSlotTypeVersionsError>;

    /// <p>Returns slot type information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all slot types that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all slot types. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetSlotTypes</code> action. </p>
    fn get_slot_types(
        &self,
        input: GetSlotTypesRequest,
    ) -> RusotoFuture<GetSlotTypesResponse, GetSlotTypesError>;

    /// <p>Use the <code>GetUtterancesView</code> operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to.</p> <p>For example, say that you have created a bot to order flowers. After your users have used your bot for a while, use the <code>GetUtterancesView</code> operation to see the requests that they have made and whether they have been successful. You might find that the utterance "I want flowers" is not being recognized. You could add this utterance to the <code>OrderFlowers</code> intent so that your bot recognizes that utterance.</p> <p>After you publish a new version of a bot, you can get information about the old version and the new so that you can compare the performance across the two versions. </p> <note> <p>Utterance statistics are generated once a day. Data is available for the last 15 days. You can request information for up to 5 versions in each request. The response contains information about a maximum of 100 utterances for each version.</p> </note> <p>This operation requires permissions for the <code>lex:GetUtterancesView</code> action.</p>
    fn get_utterances_view(
        &self,
        input: GetUtterancesViewRequest,
    ) -> RusotoFuture<GetUtterancesViewResponse, GetUtterancesViewError>;

    /// <p>Creates an Amazon Lex conversational bot or replaces an existing bot. When you create or update a bot you are only required to specify a name, a locale, and whether the bot is directed toward children under age 13. You can use this to add intents later, or to remove intents from an existing bot. When you create a bot with the minimum information, the bot is created or updated but Amazon Lex returns the <code/> response <code>FAILED</code>. You can build the bot after you add one or more intents. For more information about Amazon Lex bots, see <a>how-it-works</a>. </p> <p>If you specify the name of an existing bot, the fields in the request replace the existing values in the <code>$LATEST</code> version of the bot. Amazon Lex removes any fields that you don't provide values for in the request, except for the <code>idleTTLInSeconds</code> and <code>privacySettings</code> fields, which are set to their default values. If you don't specify values for required fields, Amazon Lex throws an exception.</p> <p>This operation requires permissions for the <code>lex:PutBot</code> action. For more information, see <a>auth-and-access-control</a>.</p>
    fn put_bot(&self, input: PutBotRequest) -> RusotoFuture<PutBotResponse, PutBotError>;

    /// <p>Creates an alias for the specified version of the bot or replaces an alias for the specified bot. To change the version of the bot that the alias points to, replace the alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:PutBotAlias</code> action. </p>
    fn put_bot_alias(
        &self,
        input: PutBotAliasRequest,
    ) -> RusotoFuture<PutBotAliasResponse, PutBotAliasError>;

    /// <p>Creates an intent or replaces an existing intent.</p> <p>To define the interaction between the user and your bot, you use one or more intents. For a pizza ordering bot, for example, you would create an <code>OrderPizza</code> intent. </p> <p>To create an intent or replace an existing intent, you must provide the following:</p> <ul> <li> <p>Intent name. For example, <code>OrderPizza</code>.</p> </li> <li> <p>Sample utterances. For example, "Can I order a pizza, please." and "I want to order a pizza."</p> </li> <li> <p>Information to be gathered. You specify slot types for the information that your bot will request from the user. You can specify standard slot types, such as a date or a time, or custom slot types such as the size and crust of a pizza.</p> </li> <li> <p>How the intent will be fulfilled. You can provide a Lambda function or configure the intent to return the intent information to the client application. If you use a Lambda function, when all of the intent information is available, Amazon Lex invokes your Lambda function. If you configure your intent to return the intent information to the client application. </p> </li> </ul> <p>You can specify other optional information in the request, such as:</p> <ul> <li> <p>A confirmation prompt to ask the user to confirm an intent. For example, "Shall I order your pizza?"</p> </li> <li> <p>A conclusion statement to send to the user after the intent has been fulfilled. For example, "I placed your pizza order."</p> </li> <li> <p>A follow-up prompt that asks the user for additional activity. For example, asking "Do you want to order a drink with your pizza?"</p> </li> </ul> <p>If you specify an existing intent name to update the intent, Amazon Lex replaces the values in the <code>$LATEST</code> version of the intent with the values in the request. Amazon Lex removes fields that you don't provide in the request. If you don't specify the required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of an intent, the <code>status</code> field of any bot that uses the <code>$LATEST</code> version of the intent is set to <code>NOT_BUILT</code>.</p> <p>For more information, see <a>how-it-works</a>.</p> <p>This operation requires permissions for the <code>lex:PutIntent</code> action.</p>
    fn put_intent(
        &self,
        input: PutIntentRequest,
    ) -> RusotoFuture<PutIntentResponse, PutIntentError>;

    /// <p>Creates a custom slot type or replaces an existing custom slot type.</p> <p>To create a custom slot type, specify a name for the slot type and a set of enumeration values, which are the values that a slot of this type can assume. For more information, see <a>how-it-works</a>.</p> <p>If you specify the name of an existing slot type, the fields in the request replace the existing values in the <code>$LATEST</code> version of the slot type. Amazon Lex removes the fields that you don't provide in the request. If you don't specify required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of a slot type, if a bot uses the <code>$LATEST</code> version of an intent that contains the slot type, the bot's <code>status</code> field is set to <code>NOT_BUILT</code>.</p> <p>This operation requires permissions for the <code>lex:PutSlotType</code> action.</p>
    fn put_slot_type(
        &self,
        input: PutSlotTypeRequest,
    ) -> RusotoFuture<PutSlotTypeResponse, PutSlotTypeError>;

    /// <p>Starts a job to import a resource to Amazon Lex.</p>
    fn start_import(
        &self,
        input: StartImportRequest,
    ) -> RusotoFuture<StartImportResponse, StartImportError>;
}
/// A client for the Amazon Lex Model Building Service API.
pub struct LexModelsClient {
    client: Client,
    region: region::Region,
}

impl LexModelsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LexModelsClient {
        LexModelsClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LexModelsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        LexModelsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl LexModels for LexModelsClient {
    /// <p>Creates a new version of the bot based on the <code>$LATEST</code> version. If the <code>$LATEST</code> version of this resource hasn't changed since you created the last version, Amazon Lex doesn't create a new version. It returns the last created version.</p> <note> <p>You can update only the <code>$LATEST</code> version of the bot. You can't update the numbered versions that you create with the <code>CreateBotVersion</code> operation.</p> </note> <p> When you create the first version of a bot, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p> This operation requires permission for the <code>lex:CreateBotVersion</code> action. </p>
    fn create_bot_version(
        &self,
        input: CreateBotVersionRequest,
    ) -> RusotoFuture<CreateBotVersionResponse, CreateBotVersionError> {
        let request_uri = format!("/bots/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateBotVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBotVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new version of an intent based on the <code>$LATEST</code> version of the intent. If the <code>$LATEST</code> version of this intent hasn't changed since you last updated it, Amazon Lex doesn't create a new version. It returns the last version you created.</p> <note> <p>You can update only the <code>$LATEST</code> version of the intent. You can't update the numbered versions that you create with the <code>CreateIntentVersion</code> operation.</p> </note> <p> When you create a version of an intent, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions to perform the <code>lex:CreateIntentVersion</code> action. </p>
    fn create_intent_version(
        &self,
        input: CreateIntentVersionRequest,
    ) -> RusotoFuture<CreateIntentVersionResponse, CreateIntentVersionError> {
        let request_uri = format!("/intents/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateIntentVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateIntentVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a new version of a slot type based on the <code>$LATEST</code> version of the specified slot type. If the <code>$LATEST</code> version of this resource has not changed since the last version that you created, Amazon Lex doesn't create a new version. It returns the last version that you created. </p> <note> <p>You can update only the <code>$LATEST</code> version of a slot type. You can't update the numbered versions that you create with the <code>CreateSlotTypeVersion</code> operation.</p> </note> <p>When you create a version of a slot type, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions for the <code>lex:CreateSlotTypeVersion</code> action.</p>
    fn create_slot_type_version(
        &self,
        input: CreateSlotTypeVersionRequest,
    ) -> RusotoFuture<CreateSlotTypeVersionResponse, CreateSlotTypeVersionError> {
        let request_uri = format!("/slottypes/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateSlotTypeVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateSlotTypeVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes all versions of the bot, including the <code>$LATEST</code> version. To delete a specific version of the bot, use the <a>DeleteBotVersion</a> operation.</p> <p>If a bot has an alias, you can't delete it. Instead, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the alias that refers to the bot. To remove the reference to the bot, delete the alias. If you get the same exception again, delete the referring alias until the <code>DeleteBot</code> operation is successful.</p> <p>This operation requires permissions for the <code>lex:DeleteBot</code> action.</p>
    fn delete_bot(&self, input: DeleteBotRequest) -> RusotoFuture<(), DeleteBotError> {
        let request_uri = format!("/bots/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an alias for the specified bot. </p> <p>You can't delete an alias that is used in the association between a bot and a messaging channel. If an alias is used in a channel association, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the channel association that refers to the bot. You can remove the reference to the alias by deleting the channel association. If you get the same exception again, delete the referring association until the <code>DeleteBotAlias</code> operation is successful.</p>
    fn delete_bot_alias(
        &self,
        input: DeleteBotAliasRequest,
    ) -> RusotoFuture<(), DeleteBotAliasError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{name}",
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBotAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permission for the <code>lex:DeleteBotChannelAssociation</code> action.</p>
    fn delete_bot_channel_association(
        &self,
        input: DeleteBotChannelAssociationRequest,
    ) -> RusotoFuture<(), DeleteBotChannelAssociationError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{alias_name}/channels/{name}",
            alias_name = input.bot_alias,
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBotChannelAssociationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a specific version of a bot. To delete all versions of a bot, use the <a>DeleteBot</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteBotVersion</code> action.</p>
    fn delete_bot_version(
        &self,
        input: DeleteBotVersionRequest,
    ) -> RusotoFuture<(), DeleteBotVersionError> {
        let request_uri = format!(
            "/bots/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBotVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes all versions of the intent, including the <code>$LATEST</code> version. To delete a specific version of the intent, use the <a>DeleteIntentVersion</a> operation.</p> <p> You can delete a version of an intent only if it is not referenced. To delete an intent that is referred to in one or more bots (see <a>how-it-works</a>), you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, it provides an example reference that shows where the intent is referenced. To remove the reference to the intent, either update the bot or delete it. If you get the same exception when you attempt to delete the intent again, repeat until the intent has no references and the call to <code>DeleteIntent</code> is successful. </p> </note> <p> This operation requires permission for the <code>lex:DeleteIntent</code> action. </p>
    fn delete_intent(&self, input: DeleteIntentRequest) -> RusotoFuture<(), DeleteIntentError> {
        let request_uri = format!("/intents/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIntentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specific version of an intent. To delete all versions of a intent, use the <a>DeleteIntent</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteIntentVersion</code> action.</p>
    fn delete_intent_version(
        &self,
        input: DeleteIntentVersionRequest,
    ) -> RusotoFuture<(), DeleteIntentVersionError> {
        let request_uri = format!(
            "/intents/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteIntentVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes all versions of the slot type, including the <code>$LATEST</code> version. To delete a specific version of the slot type, use the <a>DeleteSlotTypeVersion</a> operation.</p> <p> You can delete a version of a slot type only if it is not referenced. To delete a slot type that is referred to in one or more intents, you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, the exception provides an example reference that shows the intent where the slot type is referenced. To remove the reference to the slot type, either update the intent or delete it. If you get the same exception when you attempt to delete the slot type again, repeat until the slot type has no references and the <code>DeleteSlotType</code> call is successful. </p> </note> <p>This operation requires permission for the <code>lex:DeleteSlotType</code> action.</p>
    fn delete_slot_type(
        &self,
        input: DeleteSlotTypeRequest,
    ) -> RusotoFuture<(), DeleteSlotTypeError> {
        let request_uri = format!("/slottypes/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSlotTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specific version of a slot type. To delete all versions of a slot type, use the <a>DeleteSlotType</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteSlotTypeVersion</code> action.</p>
    fn delete_slot_type_version(
        &self,
        input: DeleteSlotTypeVersionRequest,
    ) -> RusotoFuture<(), DeleteSlotTypeVersionError> {
        let request_uri = format!(
            "/slottypes/{name}/version/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteSlotTypeVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes stored utterances.</p> <p>Amazon Lex stores the utterances that users send to your bot. Utterances are stored for 15 days for use with the <a>GetUtterancesView</a> operation, and then stored indefinitely for use in improving the ability of your bot to respond to user input.</p> <p>Use the <code>DeleteStoredUtterances</code> operation to manually delete stored utterances for a specific user.</p> <p>This operation requires permissions for the <code>lex:DeleteUtterances</code> action.</p>
    fn delete_utterances(
        &self,
        input: DeleteUtterancesRequest,
    ) -> RusotoFuture<(), DeleteUtterancesError> {
        let request_uri = format!(
            "/bots/{bot_name}/utterances/{user_id}",
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUtterancesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns metadata information for a specific bot. You must provide the bot name and the bot version or alias. </p> <p> This operation requires permissions for the <code>lex:GetBot</code> action. </p>
    fn get_bot(&self, input: GetBotRequest) -> RusotoFuture<GetBotResponse, GetBotError> {
        let request_uri = format!(
            "/bots/{name}/versions/{versionoralias}",
            name = input.name,
            versionoralias = input.version_or_alias
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBotResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an Amazon Lex bot alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:GetBotAlias</code> action.</p>
    fn get_bot_alias(
        &self,
        input: GetBotAliasRequest,
    ) -> RusotoFuture<GetBotAliasResponse, GetBotAliasError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{name}",
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBotAliasResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of aliases for a specified Amazon Lex bot.</p> <p>This operation requires permissions for the <code>lex:GetBotAliases</code> action.</p>
    fn get_bot_aliases(
        &self,
        input: GetBotAliasesRequest,
    ) -> RusotoFuture<GetBotAliasesResponse, GetBotAliasesError> {
        let request_uri = format!("/bots/{bot_name}/aliases/", bot_name = input.bot_name);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_contains {
            params.put("nameContains", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBotAliasesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotAliasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permissions for the <code>lex:GetBotChannelAssociation</code> action.</p>
    fn get_bot_channel_association(
        &self,
        input: GetBotChannelAssociationRequest,
    ) -> RusotoFuture<GetBotChannelAssociationResponse, GetBotChannelAssociationError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{alias_name}/channels/{name}",
            alias_name = input.bot_alias,
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetBotChannelAssociationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBotChannelAssociationError::from_response(response))
                }))
            }
        })
    }

    /// <p> Returns a list of all of the channels associated with the specified bot. </p> <p>The <code>GetBotChannelAssociations</code> operation requires permissions for the <code>lex:GetBotChannelAssociations</code> action.</p>
    fn get_bot_channel_associations(
        &self,
        input: GetBotChannelAssociationsRequest,
    ) -> RusotoFuture<GetBotChannelAssociationsResponse, GetBotChannelAssociationsError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{alias_name}/channels/",
            alias_name = input.bot_alias,
            bot_name = input.bot_name
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_contains {
            params.put("nameContains", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetBotChannelAssociationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetBotChannelAssociationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about all of the versions of a bot.</p> <p>The <code>GetBotVersions</code> operation returns a <code>BotMetadata</code> object for each version of a bot. For example, if a bot has three numbered versions, the <code>GetBotVersions</code> operation returns four <code>BotMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetBotVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetBotVersions</code> action.</p>
    fn get_bot_versions(
        &self,
        input: GetBotVersionsRequest,
    ) -> RusotoFuture<GetBotVersionsResponse, GetBotVersionsError> {
        let request_uri = format!("/bots/{name}/versions/", name = input.name);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBotVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns bot information as follows: </p> <ul> <li> <p>If you provide the <code>nameContains</code> field, the response includes information for the <code>$LATEST</code> version of all bots whose name contains the specified string.</p> </li> <li> <p>If you don't specify the <code>nameContains</code> field, the operation returns information about the <code>$LATEST</code> version of all of your bots.</p> </li> </ul> <p>This operation requires permission for the <code>lex:GetBots</code> action.</p>
    fn get_bots(&self, input: GetBotsRequest) -> RusotoFuture<GetBotsResponse, GetBotsError> {
        let request_uri = "/bots/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_contains {
            params.put("nameContains", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBotsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBotsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a built-in intent.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntent</code> action.</p>
    fn get_builtin_intent(
        &self,
        input: GetBuiltinIntentRequest,
    ) -> RusotoFuture<GetBuiltinIntentResponse, GetBuiltinIntentError> {
        let request_uri = format!("/builtins/intents/{signature}", signature = input.signature);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetBuiltinIntentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBuiltinIntentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of built-in intents that meet the specified criteria.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntents</code> action.</p>
    fn get_builtin_intents(
        &self,
        input: GetBuiltinIntentsRequest,
    ) -> RusotoFuture<GetBuiltinIntentsResponse, GetBuiltinIntentsError> {
        let request_uri = "/builtins/intents/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.locale {
            params.put("locale", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.signature_contains {
            params.put("signatureContains", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetBuiltinIntentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBuiltinIntentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of built-in slot types that meet the specified criteria.</p> <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p> <p>This operation requires permission for the <code>lex:GetBuiltInSlotTypes</code> action.</p>
    fn get_builtin_slot_types(
        &self,
        input: GetBuiltinSlotTypesRequest,
    ) -> RusotoFuture<GetBuiltinSlotTypesResponse, GetBuiltinSlotTypesError> {
        let request_uri = "/builtins/slottypes/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.locale {
            params.put("locale", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.signature_contains {
            params.put("signatureContains", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetBuiltinSlotTypesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBuiltinSlotTypesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Exports the contents of a Amazon Lex resource in a specified format. </p>
    fn get_export(
        &self,
        input: GetExportRequest,
    ) -> RusotoFuture<GetExportResponse, GetExportError> {
        let request_uri = "/exports/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        params.put("exportType", &input.export_type);
        params.put("name", &input.name);
        params.put("resourceType", &input.resource_type);
        params.put("version", &input.version);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetExportResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about an import job started with the <code>StartImport</code> operation.</p>
    fn get_import(
        &self,
        input: GetImportRequest,
    ) -> RusotoFuture<GetImportResponse, GetImportError> {
        let request_uri = format!("/imports/{import_id}", import_id = input.import_id);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetImportResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetImportError::from_response(response))),
                )
            }
        })
    }

    /// <p> Returns information about an intent. In addition to the intent name, you must specify the intent version. </p> <p> This operation requires permissions to perform the <code>lex:GetIntent</code> action. </p>
    fn get_intent(
        &self,
        input: GetIntentRequest,
    ) -> RusotoFuture<GetIntentResponse, GetIntentError> {
        let request_uri = format!(
            "/intents/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetIntentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about all of the versions of an intent.</p> <p>The <code>GetIntentVersions</code> operation returns an <code>IntentMetadata</code> object for each version of an intent. For example, if an intent has three numbered versions, the <code>GetIntentVersions</code> operation returns four <code>IntentMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetIntentVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetIntentVersions</code> action.</p>
    fn get_intent_versions(
        &self,
        input: GetIntentVersionsRequest,
    ) -> RusotoFuture<GetIntentVersionsResponse, GetIntentVersionsError> {
        let request_uri = format!("/intents/{name}/versions/", name = input.name);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetIntentVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntentVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns intent information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all intents that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all intents. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetIntents</code> action. </p>
    fn get_intents(
        &self,
        input: GetIntentsRequest,
    ) -> RusotoFuture<GetIntentsResponse, GetIntentsError> {
        let request_uri = "/intents/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_contains {
            params.put("nameContains", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetIntentsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific version of a slot type. In addition to specifying the slot type name, you must specify the slot type version.</p> <p>This operation requires permissions for the <code>lex:GetSlotType</code> action.</p>
    fn get_slot_type(
        &self,
        input: GetSlotTypeRequest,
    ) -> RusotoFuture<GetSlotTypeResponse, GetSlotTypeError> {
        let request_uri = format!(
            "/slottypes/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetSlotTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSlotTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about all versions of a slot type.</p> <p>The <code>GetSlotTypeVersions</code> operation returns a <code>SlotTypeMetadata</code> object for each version of a slot type. For example, if a slot type has three numbered versions, the <code>GetSlotTypeVersions</code> operation returns four <code>SlotTypeMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetSlotTypeVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetSlotTypeVersions</code> action.</p>
    fn get_slot_type_versions(
        &self,
        input: GetSlotTypeVersionsRequest,
    ) -> RusotoFuture<GetSlotTypeVersionsResponse, GetSlotTypeVersionsError> {
        let request_uri = format!("/slottypes/{name}/versions/", name = input.name);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetSlotTypeVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSlotTypeVersionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns slot type information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all slot types that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all slot types. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetSlotTypes</code> action. </p>
    fn get_slot_types(
        &self,
        input: GetSlotTypesRequest,
    ) -> RusotoFuture<GetSlotTypesResponse, GetSlotTypesError> {
        let request_uri = "/slottypes/";

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_contains {
            params.put("nameContains", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetSlotTypesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSlotTypesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use the <code>GetUtterancesView</code> operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to.</p> <p>For example, say that you have created a bot to order flowers. After your users have used your bot for a while, use the <code>GetUtterancesView</code> operation to see the requests that they have made and whether they have been successful. You might find that the utterance "I want flowers" is not being recognized. You could add this utterance to the <code>OrderFlowers</code> intent so that your bot recognizes that utterance.</p> <p>After you publish a new version of a bot, you can get information about the old version and the new so that you can compare the performance across the two versions. </p> <note> <p>Utterance statistics are generated once a day. Data is available for the last 15 days. You can request information for up to 5 versions in each request. The response contains information about a maximum of 100 utterances for each version.</p> </note> <p>This operation requires permissions for the <code>lex:GetUtterancesView</code> action.</p>
    fn get_utterances_view(
        &self,
        input: GetUtterancesViewRequest,
    ) -> RusotoFuture<GetUtterancesViewResponse, GetUtterancesViewError> {
        let request_uri = format!("/bots/{botname}/utterances", botname = input.bot_name);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut params = Params::new();
        for item in input.bot_versions.iter() {
            params.put("bot_versions", item);
        }
        params.put("status_type", &input.status_type);
        params.put("view", "aggregation");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetUtterancesViewResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUtterancesViewError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon Lex conversational bot or replaces an existing bot. When you create or update a bot you are only required to specify a name, a locale, and whether the bot is directed toward children under age 13. You can use this to add intents later, or to remove intents from an existing bot. When you create a bot with the minimum information, the bot is created or updated but Amazon Lex returns the <code/> response <code>FAILED</code>. You can build the bot after you add one or more intents. For more information about Amazon Lex bots, see <a>how-it-works</a>. </p> <p>If you specify the name of an existing bot, the fields in the request replace the existing values in the <code>$LATEST</code> version of the bot. Amazon Lex removes any fields that you don't provide values for in the request, except for the <code>idleTTLInSeconds</code> and <code>privacySettings</code> fields, which are set to their default values. If you don't specify values for required fields, Amazon Lex throws an exception.</p> <p>This operation requires permissions for the <code>lex:PutBot</code> action. For more information, see <a>auth-and-access-control</a>.</p>
    fn put_bot(&self, input: PutBotRequest) -> RusotoFuture<PutBotResponse, PutBotError> {
        let request_uri = format!("/bots/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutBotResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an alias for the specified version of the bot or replaces an alias for the specified bot. To change the version of the bot that the alias points to, replace the alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:PutBotAlias</code> action. </p>
    fn put_bot_alias(
        &self,
        input: PutBotAliasRequest,
    ) -> RusotoFuture<PutBotAliasResponse, PutBotAliasError> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{name}",
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutBotAliasResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutBotAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an intent or replaces an existing intent.</p> <p>To define the interaction between the user and your bot, you use one or more intents. For a pizza ordering bot, for example, you would create an <code>OrderPizza</code> intent. </p> <p>To create an intent or replace an existing intent, you must provide the following:</p> <ul> <li> <p>Intent name. For example, <code>OrderPizza</code>.</p> </li> <li> <p>Sample utterances. For example, "Can I order a pizza, please." and "I want to order a pizza."</p> </li> <li> <p>Information to be gathered. You specify slot types for the information that your bot will request from the user. You can specify standard slot types, such as a date or a time, or custom slot types such as the size and crust of a pizza.</p> </li> <li> <p>How the intent will be fulfilled. You can provide a Lambda function or configure the intent to return the intent information to the client application. If you use a Lambda function, when all of the intent information is available, Amazon Lex invokes your Lambda function. If you configure your intent to return the intent information to the client application. </p> </li> </ul> <p>You can specify other optional information in the request, such as:</p> <ul> <li> <p>A confirmation prompt to ask the user to confirm an intent. For example, "Shall I order your pizza?"</p> </li> <li> <p>A conclusion statement to send to the user after the intent has been fulfilled. For example, "I placed your pizza order."</p> </li> <li> <p>A follow-up prompt that asks the user for additional activity. For example, asking "Do you want to order a drink with your pizza?"</p> </li> </ul> <p>If you specify an existing intent name to update the intent, Amazon Lex replaces the values in the <code>$LATEST</code> version of the intent with the values in the request. Amazon Lex removes fields that you don't provide in the request. If you don't specify the required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of an intent, the <code>status</code> field of any bot that uses the <code>$LATEST</code> version of the intent is set to <code>NOT_BUILT</code>.</p> <p>For more information, see <a>how-it-works</a>.</p> <p>This operation requires permissions for the <code>lex:PutIntent</code> action.</p>
    fn put_intent(
        &self,
        input: PutIntentRequest,
    ) -> RusotoFuture<PutIntentResponse, PutIntentError> {
        let request_uri = format!("/intents/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutIntentResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutIntentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a custom slot type or replaces an existing custom slot type.</p> <p>To create a custom slot type, specify a name for the slot type and a set of enumeration values, which are the values that a slot of this type can assume. For more information, see <a>how-it-works</a>.</p> <p>If you specify the name of an existing slot type, the fields in the request replace the existing values in the <code>$LATEST</code> version of the slot type. Amazon Lex removes the fields that you don't provide in the request. If you don't specify required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of a slot type, if a bot uses the <code>$LATEST</code> version of an intent that contains the slot type, the bot's <code>status</code> field is set to <code>NOT_BUILT</code>.</p> <p>This operation requires permissions for the <code>lex:PutSlotType</code> action.</p>
    fn put_slot_type(
        &self,
        input: PutSlotTypeRequest,
    ) -> RusotoFuture<PutSlotTypeResponse, PutSlotTypeError> {
        let request_uri = format!("/slottypes/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<PutSlotTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutSlotTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a job to import a resource to Amazon Lex.</p>
    fn start_import(
        &self,
        input: StartImportRequest,
    ) -> RusotoFuture<StartImportResponse, StartImportError> {
        let request_uri = "/imports/";

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<StartImportResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartImportError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
