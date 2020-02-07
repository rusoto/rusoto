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
/// <p>Provides information about a bot alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Settings that determine how Amazon Lex uses conversation logs for the alias.</p>
    #[serde(rename = "conversationLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_logs: Option<ConversationLogsResponse>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BuiltinIntentSlot {
    /// <p>A list of the slots defined for the intent.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Provides information about a built in slot type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides the settings needed for conversation logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConversationLogsRequest {
    /// <p>The Amazon Resource Name (ARN) of an IAM role with permission to write to your CloudWatch Logs for text logs and your S3 bucket for audio logs. If audio encryption is enabled, this role also provides access permission for the AWS KMS key used for encrypting audio logs. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/conversation-logs-role-and-policy.html">Creating an IAM Role and Policy for Conversation Logs</a>.</p>
    #[serde(rename = "iamRoleArn")]
    pub iam_role_arn: String,
    /// <p>The settings for your conversation logs. You can log the conversation text, conversation audio, or both.</p>
    #[serde(rename = "logSettings")]
    pub log_settings: Vec<LogSettingsRequest>,
}

/// <p>Contains information about conversation log settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConversationLogsResponse {
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to write your logs to CloudWatch Logs or an S3 bucket.</p>
    #[serde(rename = "iamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The settings for your conversation logs. You can log text, audio, or both.</p>
    #[serde(rename = "logSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_settings: Option<Vec<LogSettingsResponse>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Indicates whether utterances entered by the user should be sent to Amazon Comprehend for sentiment analysis.</p>
    #[serde(rename = "detectSentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_sentiment: Option<bool>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBotAliasRequest {
    /// <p>The name of the bot that the alias points to.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the alias to delete. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBotRequest {
    /// <p>The name of the bot. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBotVersionRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the bot to delete. You cannot delete the <code>$LATEST</code> version of the bot. To delete the <code>$LATEST</code> version, use the <a>DeleteBot</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIntentRequest {
    /// <p>The name of the intent. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIntentVersionRequest {
    /// <p>The name of the intent.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the intent to delete. You cannot delete the <code>$LATEST</code> version of the intent. To delete the <code>$LATEST</code> version, use the <a>DeleteIntent</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSlotTypeRequest {
    /// <p>The name of the slot type. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSlotTypeVersionRequest {
    /// <p>The name of the slot type.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the slot type to delete. You cannot delete the <code>$LATEST</code> version of the slot type. To delete the <code>$LATEST</code> version, use the <a>DeleteSlotType</a> operation.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBotAliasRequest {
    /// <p>The name of the bot.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>The name of the bot alias. The name is case sensitive.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The settings that determine how Amazon Lex uses conversation logs for the alias.</p>
    #[serde(rename = "conversationLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_logs: Option<ConversationLogsResponse>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBotRequest {
    /// <p>The name of the bot. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version or alias of the bot.</p>
    #[serde(rename = "versionOrAlias")]
    pub version_or_alias: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Indicates whether user utterances should be sent to Amazon Comprehend for sentiment analysis.</p>
    #[serde(rename = "detectSentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_sentiment: Option<bool>,
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
    /// <p>The status of the bot. </p> <p>When the status is <code>BUILDING</code> Amazon Lex is building the bot for testing and use.</p> <p>If the status of the bot is <code>READY_BASIC_TESTING</code>, you can test the bot using the exact utterances specified in the bot's intents. When the bot is ready for full testing or to run, the status is <code>READY</code>.</p> <p>If there was a problem with building the bot, the status is <code>FAILED</code> and the <code>failureReason</code> field explains why the bot did not build.</p> <p>If the bot was saved but not built, the status is <code>NOT_BUILT</code>.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBuiltinIntentRequest {
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[serde(rename = "signature")]
    pub signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImportRequest {
    /// <p>The identifier of the import job information to return.</p>
    #[serde(rename = "importId")]
    pub import_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIntentRequest {
    /// <p>The name of the intent. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the intent.</p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSlotTypeRequest {
    /// <p>The name of the slot type. The name is case sensitive. </p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The version of the slot type. </p>
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUtterancesViewRequest {
    /// <p>The name of the bot for which utterance information should be returned.</p>
    #[serde(rename = "botName")]
    pub bot_name: String,
    /// <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    #[serde(rename = "botVersions")]
    pub bot_versions: Vec<String>,
    /// <p>To return utterances that were recognized and handled, use <code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    #[serde(rename = "statusType")]
    pub status_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUtterancesViewResponse {
    /// <p>The name of the bot for which utterance information was returned.</p>
    #[serde(rename = "botName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    /// <p>An array of <a>UtteranceList</a> objects, each containing a list of <a>UtteranceData</a> objects describing the utterances that were processed by your bot. The response contains a maximum of 100 <code>UtteranceData</code> objects for each version. Amazon Lex returns the most frequent utterances received by the bot in the last 15 days.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Settings used to configure delivery mode and destination for conversation logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LogSettingsRequest {
    /// <p>Where the logs will be delivered. Text logs are delivered to a CloudWatch Logs log group. Audio logs are delivered to an S3 bucket.</p>
    #[serde(rename = "destination")]
    pub destination: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS KMS customer managed key for encrypting audio logs delivered to an S3 bucket. The key does not apply to CloudWatch Logs and is optional for S3 buckets.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The type of logging to enable. Text logs are delivered to a CloudWatch Logs log group. Audio logs are delivered to an S3 bucket.</p>
    #[serde(rename = "logType")]
    pub log_type: String,
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch Logs log group or S3 bucket where the logs should be delivered.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>The settings for conversation logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogSettingsResponse {
    /// <p>The destination where logs are delivered.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the key used to encrypt audio logs in an S3 bucket.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The type of logging that is enabled.</p>
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch Logs log group or S3 bucket where the logs are delivered.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The resource prefix is the first part of the S3 object key within the S3 bucket that you specified to contain audio logs. For CloudWatch Logs it is the prefix of the log stream name within the log group that you specified. </p>
    #[serde(rename = "resourcePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_prefix: Option<String>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Settings for conversation logs for the alias.</p>
    #[serde(rename = "conversationLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_logs: Option<ConversationLogsRequest>,
    /// <p>A description of the alias.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the alias. The name is <i>not</i> case sensitive.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The settings that determine how Amazon Lex uses conversation logs for the alias.</p>
    #[serde(rename = "conversationLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_logs: Option<ConversationLogsResponse>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBotRequest {
    /// <p>When Amazon Lex can't understand the user's input in context, it tries to elicit the information a few times. After that, Amazon Lex sends the message defined in <code>abortStatement</code> to the user, and then aborts the conversation. To set the number of retries, use the <code>valueElicitationPrompt</code> field for the slot type. </p> <p>For example, in a pizza ordering bot, Amazon Lex might ask a user "What type of crust would you like?" If the user's response is not one of the expected responses (for example, "thin crust, "deep dish," etc.), Amazon Lex tries to elicit a correct response a few more times. </p> <p>For example, in a pizza ordering application, <code>OrderPizza</code> might be one of the intents. This intent might require the <code>CrustType</code> slot. You specify the <code>valueElicitationPrompt</code> field when you create the <code>CrustType</code> slot.</p> <p>If you have defined a fallback intent the abort statement will not be sent to the user, the fallback intent is used instead. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/built-in-intent-fallback.html"> AMAZON.FallbackIntent</a>.</p>
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
    /// <p><p>When Amazon Lex doesn&#39;t understand the user&#39;s intent, it uses this message to get clarification. To specify how many times Amazon Lex should repeat the clarification prompt, use the <code>maxAttempts</code> field. If Amazon Lex still doesn&#39;t understand, it sends the message in the <code>abortStatement</code> field. </p> <p>When you create a clarification prompt, make sure that it suggests the correct response from the user. for example, for a bot that orders pizza and drinks, you might create this clarification prompt: &quot;What would you like to do? You can say &#39;Order a pizza&#39; or &#39;Order a drink.&#39;&quot;</p> <p>If you have defined a fallback intent, it will be invoked if the clarification prompt is repeated the number of times defined in the <code>maxAttempts</code> field. For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/built-in-intent-fallback.html"> AMAZON.FallbackIntent</a>.</p> <p>If you don&#39;t define a clarification prompt, at runtime Amazon Lex will return a 400 Bad Request exception in three cases: </p> <ul> <li> <p>Follow-up prompt - When the user responds to a follow-up prompt but does not provide an intent. For example, in response to a follow-up prompt that says &quot;Would you like anything else today?&quot; the user says &quot;Yes.&quot; Amazon Lex will return a 400 Bad Request exception because it does not have a clarification prompt to send to the user to get an intent.</p> </li> <li> <p>Lambda function - When using a Lambda function, you return an <code>ElicitIntent</code> dialog type. Since Amazon Lex does not have a clarification prompt to get an intent from the user, it returns a 400 Bad Request exception.</p> </li> <li> <p>PutSession operation - When using the <code>PutSession</code> operation, you send an <code>ElicitIntent</code> dialog type. Since Amazon Lex does not have a clarification prompt to get an intent from the user, it returns a 400 Bad Request exception.</p> </li> </ul></p>
    #[serde(rename = "clarificationPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clarification_prompt: Option<Prompt>,
    /// <p>When set to <code>true</code> a new numbered version of the bot is created. This is the same as calling the <code>CreateBotVersion</code> operation. If you don't specify <code>createVersion</code>, the default is <code>false</code>.</p>
    #[serde(rename = "createVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_version: Option<bool>,
    /// <p>A description of the bot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When set to <code>true</code> user utterances are sent to Amazon Comprehend for sentiment analysis. If you don't specify <code>detectSentiment</code>, the default is <code>false</code>.</p>
    #[serde(rename = "detectSentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_sentiment: Option<bool>,
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
    /// <p>The Amazon Polly voice ID that you want Amazon Lex to use for voice interactions with the user. The locale configured for the voice must match the locale of the bot. For more information, see <a href="https://docs.aws.amazon.com/polly/latest/dg/voicelist.html">Voices in Amazon Polly</a> in the <i>Amazon Polly Developer Guide</i>.</p>
    #[serde(rename = "voiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p> <code>True</code> if a new version of the bot was created. If the <code>createVersion</code> field was not specified in the request, the <code>createVersion</code> field is set to false in the response.</p>
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
    /// <p> <code>true</code> if the bot is configured to send user utterances to Amazon Comprehend for sentiment analysis. If the <code>detectSentiment</code> field was not specified in the request, the <code>detectSentiment</code> field is <code>false</code> in the response.</p>
    #[serde(rename = "detectSentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_sentiment: Option<bool>,
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
    /// <p> When you send a request to create a bot with <code>processBehavior</code> set to <code>BUILD</code>, Amazon Lex sets the <code>status</code> response element to <code>BUILDING</code>.</p> <p>In the <code>READY_BASIC_TESTING</code> state you can test the bot with user inputs that exactly match the utterances configured for the bot's intents and values in the slot types.</p> <p>If Amazon Lex can't build the bot, Amazon Lex sets <code>status</code> to <code>FAILED</code>. Amazon Lex returns the reason for the failure in the <code>failureReason</code> response element. </p> <p>When you set <code>processBehavior</code> to <code>SAVE</code>, Amazon Lex sets the status code to <code>NOT BUILT</code>.</p> <p>When the bot is in the <code>READY</code> state you can test and publish the bot.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>When set to <code>true</code> a new numbered version of the intent is created. This is the same as calling the <code>CreateIntentVersion</code> operation. If you do not specify <code>createVersion</code>, the default is <code>false</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p> <code>True</code> if a new version of the intent was created. If the <code>createVersion</code> field was not specified in the request, the <code>createVersion</code> field is set to false in the response.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSlotTypeRequest {
    /// <p>Identifies a specific revision of the <code>$LATEST</code> version.</p> <p>When you create a new slot type, leave the <code>checksum</code> field blank. If you specify a checksum you get a <code>BadRequestException</code> exception.</p> <p>When you want to update a slot type, set the <code>checksum</code> field to the checksum of the most recent revision of the <code>$LATEST</code> version. If you don't specify the <code> checksum</code> field, or if the checksum does not match the <code>$LATEST</code> version, you get a <code>PreconditionFailedException</code> exception.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>When set to <code>true</code> a new numbered version of the slot type is created. This is the same as calling the <code>CreateSlotTypeVersion</code> operation. If you do not specify <code>createVersion</code>, the default is <code>false</code>.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSlotTypeResponse {
    /// <p>Checksum of the <code>$LATEST</code> version of the slot type.</p>
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p> <code>True</code> if a new version of the slot type was created. If the <code>createVersion</code> field was not specified in the request, the <code>createVersion</code> field is set to false in the response.</p>
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
    /// <p>Determines whether a slot is obfuscated in conversation logs and stored utterances. When you obfuscate a slot, the value is replaced by the slot name in curly braces ({}). For example, if the slot name is "full_name", obfuscated values are replaced with "{full_name}". For more information, see <a href="https://docs.aws.amazon.com/lex/latest/dg/how-obfuscate.html"> Slot Obfuscation </a>. </p>
    #[serde(rename = "obfuscationSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub payload: bytes::Bytes,
    /// <p><p>Specifies the type of resource to export. Each resource also exports any resources that it depends on. </p> <ul> <li> <p>A bot exports dependent intents.</p> </li> <li> <p>An intent exports dependent slot types.</p> </li> </ul></p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl CreateBotVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBotVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBotVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateBotVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateBotVersionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBotVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateBotVersionError::NotFound(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(CreateBotVersionError::PreconditionFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBotVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBotVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateBotVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateBotVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateBotVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBotVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateBotVersionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBotVersionError {}
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
}

impl CreateIntentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIntentVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateIntentVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateIntentVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateIntentVersionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateIntentVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateIntentVersionError::NotFound(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(CreateIntentVersionError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIntentVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIntentVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateIntentVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateIntentVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateIntentVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateIntentVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateIntentVersionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIntentVersionError {}
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
}

impl CreateSlotTypeVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSlotTypeVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::NotFound(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(CreateSlotTypeVersionError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSlotTypeVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSlotTypeVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateSlotTypeVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateSlotTypeVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateSlotTypeVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSlotTypeVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateSlotTypeVersionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSlotTypeVersionError {}
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
}

impl DeleteBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBotError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteBotError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBotError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteBotError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBotError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteBotError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBotError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteBotError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteBotError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBotError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBotError {}
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
}

impl DeleteBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBotAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBotAliasError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteBotAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBotAliasError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteBotAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBotAliasError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteBotAliasError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBotAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBotAliasError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBotAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteBotAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBotAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteBotAliasError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBotAliasError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBotAliasError {}
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
}

impl DeleteBotChannelAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteBotChannelAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBotChannelAssociationError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteBotChannelAssociationError::Conflict(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBotChannelAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteBotChannelAssociationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBotChannelAssociationError::NotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBotChannelAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBotChannelAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBotChannelAssociationError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteBotChannelAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBotChannelAssociationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteBotChannelAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBotChannelAssociationError {}
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
}

impl DeleteBotVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBotVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBotVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteBotVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteBotVersionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteBotVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBotVersionError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteBotVersionError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBotVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBotVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBotVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteBotVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteBotVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteBotVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBotVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBotVersionError {}
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
}

impl DeleteIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteIntentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteIntentError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteIntentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteIntentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntentError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteIntentError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIntentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIntentError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteIntentError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteIntentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteIntentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteIntentError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteIntentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIntentError {}
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
}

impl DeleteIntentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntentVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteIntentVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteIntentVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteIntentVersionError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteIntentVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntentVersionError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteIntentVersionError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIntentVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIntentVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteIntentVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteIntentVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteIntentVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteIntentVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteIntentVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIntentVersionError {}
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
}

impl DeleteSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSlotTypeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSlotTypeError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteSlotTypeError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteSlotTypeError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteSlotTypeError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSlotTypeError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSlotTypeError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSlotTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSlotTypeError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSlotTypeError {}
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
}

impl DeleteSlotTypeVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSlotTypeVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::NotFound(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSlotTypeVersionError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSlotTypeVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSlotTypeVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeVersionError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSlotTypeVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSlotTypeVersionError {}
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
}

impl DeleteUtterancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUtterancesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteUtterancesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteUtterancesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteUtterancesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUtterancesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUtterancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUtterancesError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteUtterancesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteUtterancesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteUtterancesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUtterancesError {}
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
}

impl GetBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotError::LimitExceeded(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetBotError::NotFound(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBotError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotError {}
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
}

impl GetBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotAliasError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotAliasError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBotAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotAliasError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBotAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotAliasError {}
/// Errors returned by GetBotAliases
#[derive(Debug, PartialEq)]
pub enum GetBotAliasesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl GetBotAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotAliasesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotAliasesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotAliasesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotAliasesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotAliasesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotAliasesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotAliasesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotAliasesError {}
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
}

impl GetBotChannelAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotChannelAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotChannelAssociationError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotChannelAssociationError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotChannelAssociationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBotChannelAssociationError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotChannelAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotChannelAssociationError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotChannelAssociationError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotChannelAssociationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBotChannelAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotChannelAssociationError {}
/// Errors returned by GetBotChannelAssociations
#[derive(Debug, PartialEq)]
pub enum GetBotChannelAssociationsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl GetBotChannelAssociationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotChannelAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotChannelAssociationsError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotChannelAssociationsError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotChannelAssociationsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotChannelAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotChannelAssociationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotChannelAssociationsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotChannelAssociationsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotChannelAssociationsError {}
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
}

impl GetBotVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotVersionsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotVersionsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotVersionsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBotVersionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotVersionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBotVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotVersionsError {}
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
}

impl GetBotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBotsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBotsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBotsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBotsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBotsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBotsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBotsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBotsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBotsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBotsError {}
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
}

impl GetBuiltinIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBuiltinIntentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBuiltinIntentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBuiltinIntentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBuiltinIntentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBuiltinIntentError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBuiltinIntentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBuiltinIntentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBuiltinIntentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBuiltinIntentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetBuiltinIntentError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBuiltinIntentError {}
/// Errors returned by GetBuiltinIntents
#[derive(Debug, PartialEq)]
pub enum GetBuiltinIntentsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl GetBuiltinIntentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBuiltinIntentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBuiltinIntentsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBuiltinIntentsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBuiltinIntentsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBuiltinIntentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBuiltinIntentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBuiltinIntentsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBuiltinIntentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBuiltinIntentsError {}
/// Errors returned by GetBuiltinSlotTypes
#[derive(Debug, PartialEq)]
pub enum GetBuiltinSlotTypesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl GetBuiltinSlotTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBuiltinSlotTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBuiltinSlotTypesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetBuiltinSlotTypesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetBuiltinSlotTypesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBuiltinSlotTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBuiltinSlotTypesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBuiltinSlotTypesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetBuiltinSlotTypesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBuiltinSlotTypesError {}
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
}

impl GetExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetExportError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetExportError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetExportError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetExportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetExportError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetExportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetExportError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExportError {}
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
}

impl GetImportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetImportError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetImportError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetImportError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetImportError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetImportError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetImportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetImportError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImportError {}
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
}

impl GetIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIntentError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetIntentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetIntentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntentError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIntentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetIntentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetIntentError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntentError {}
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
}

impl GetIntentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntentVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIntentVersionsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetIntentVersionsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetIntentVersionsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntentVersionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntentVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntentVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIntentVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetIntentVersionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetIntentVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntentVersionsError {}
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
}

impl GetIntentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetIntentsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetIntentsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetIntentsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntentsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIntentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIntentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetIntentsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetIntentsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetIntentsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIntentsError {}
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
}

impl GetSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSlotTypeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSlotTypeError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSlotTypeError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSlotTypeError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSlotTypeError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSlotTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSlotTypeError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSlotTypeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSlotTypeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSlotTypeError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSlotTypeError {}
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
}

impl GetSlotTypeVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSlotTypeVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSlotTypeVersionsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSlotTypeVersionsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSlotTypeVersionsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSlotTypeVersionsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSlotTypeVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSlotTypeVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSlotTypeVersionsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSlotTypeVersionsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSlotTypeVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSlotTypeVersionsError {}
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
}

impl GetSlotTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSlotTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSlotTypesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSlotTypesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetSlotTypesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSlotTypesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSlotTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSlotTypesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSlotTypesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetSlotTypesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetSlotTypesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSlotTypesError {}
/// Errors returned by GetUtterancesView
#[derive(Debug, PartialEq)]
pub enum GetUtterancesViewError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl GetUtterancesViewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUtterancesViewError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUtterancesViewError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetUtterancesViewError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetUtterancesViewError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUtterancesViewError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUtterancesViewError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetUtterancesViewError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetUtterancesViewError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUtterancesViewError {}
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
}

impl PutBotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutBotError::BadRequest(err.msg))
                }
                "ConflictException" => return RusotoError::Service(PutBotError::Conflict(err.msg)),
                "InternalFailureException" => {
                    return RusotoError::Service(PutBotError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutBotError::LimitExceeded(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PutBotError::PreconditionFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutBotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutBotError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutBotError::Conflict(ref cause) => write!(f, "{}", cause),
            PutBotError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutBotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutBotError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutBotError {}
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
}

impl PutBotAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutBotAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutBotAliasError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutBotAliasError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PutBotAliasError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutBotAliasError::LimitExceeded(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PutBotAliasError::PreconditionFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutBotAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutBotAliasError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutBotAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            PutBotAliasError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutBotAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutBotAliasError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutBotAliasError {}
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
}

impl PutIntentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutIntentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutIntentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutIntentError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PutIntentError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutIntentError::LimitExceeded(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PutIntentError::PreconditionFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutIntentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutIntentError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutIntentError::Conflict(ref cause) => write!(f, "{}", cause),
            PutIntentError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutIntentError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutIntentError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutIntentError {}
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
}

impl PutSlotTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSlotTypeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutSlotTypeError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutSlotTypeError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(PutSlotTypeError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutSlotTypeError::LimitExceeded(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PutSlotTypeError::PreconditionFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutSlotTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSlotTypeError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutSlotTypeError::Conflict(ref cause) => write!(f, "{}", cause),
            PutSlotTypeError::InternalFailure(ref cause) => write!(f, "{}", cause),
            PutSlotTypeError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutSlotTypeError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSlotTypeError {}
/// Errors returned by StartImport
#[derive(Debug, PartialEq)]
pub enum StartImportError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and try again.</p>
    BadRequest(String),
    /// <p>An internal Amazon Lex error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
}

impl StartImportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartImportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartImportError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartImportError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartImportError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartImportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartImportError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartImportError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartImportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartImportError {}
/// Trait representing the capabilities of the Amazon Lex Model Building Service API. Amazon Lex Model Building Service clients implement this trait.
#[async_trait]
pub trait LexModels {
    /// <p>Creates a new version of the bot based on the <code>$LATEST</code> version. If the <code>$LATEST</code> version of this resource hasn't changed since you created the last version, Amazon Lex doesn't create a new version. It returns the last created version.</p> <note> <p>You can update only the <code>$LATEST</code> version of the bot. You can't update the numbered versions that you create with the <code>CreateBotVersion</code> operation.</p> </note> <p> When you create the first version of a bot, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p> This operation requires permission for the <code>lex:CreateBotVersion</code> action. </p>
    async fn create_bot_version(
        &self,
        input: CreateBotVersionRequest,
    ) -> Result<CreateBotVersionResponse, RusotoError<CreateBotVersionError>>;

    /// <p>Creates a new version of an intent based on the <code>$LATEST</code> version of the intent. If the <code>$LATEST</code> version of this intent hasn't changed since you last updated it, Amazon Lex doesn't create a new version. It returns the last version you created.</p> <note> <p>You can update only the <code>$LATEST</code> version of the intent. You can't update the numbered versions that you create with the <code>CreateIntentVersion</code> operation.</p> </note> <p> When you create a version of an intent, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions to perform the <code>lex:CreateIntentVersion</code> action. </p>
    async fn create_intent_version(
        &self,
        input: CreateIntentVersionRequest,
    ) -> Result<CreateIntentVersionResponse, RusotoError<CreateIntentVersionError>>;

    /// <p>Creates a new version of a slot type based on the <code>$LATEST</code> version of the specified slot type. If the <code>$LATEST</code> version of this resource has not changed since the last version that you created, Amazon Lex doesn't create a new version. It returns the last version that you created. </p> <note> <p>You can update only the <code>$LATEST</code> version of a slot type. You can't update the numbered versions that you create with the <code>CreateSlotTypeVersion</code> operation.</p> </note> <p>When you create a version of a slot type, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions for the <code>lex:CreateSlotTypeVersion</code> action.</p>
    async fn create_slot_type_version(
        &self,
        input: CreateSlotTypeVersionRequest,
    ) -> Result<CreateSlotTypeVersionResponse, RusotoError<CreateSlotTypeVersionError>>;

    /// <p>Deletes all versions of the bot, including the <code>$LATEST</code> version. To delete a specific version of the bot, use the <a>DeleteBotVersion</a> operation. The <code>DeleteBot</code> operation doesn't immediately remove the bot schema. Instead, it is marked for deletion and removed later.</p> <p>Amazon Lex stores utterances indefinitely for improving the ability of your bot to respond to user inputs. These utterances are not removed when the bot is deleted. To remove the utterances, use the <a>DeleteUtterances</a> operation.</p> <p>If a bot has an alias, you can't delete it. Instead, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the alias that refers to the bot. To remove the reference to the bot, delete the alias. If you get the same exception again, delete the referring alias until the <code>DeleteBot</code> operation is successful.</p> <p>This operation requires permissions for the <code>lex:DeleteBot</code> action.</p>
    async fn delete_bot(&self, input: DeleteBotRequest) -> Result<(), RusotoError<DeleteBotError>>;

    /// <p>Deletes an alias for the specified bot. </p> <p>You can't delete an alias that is used in the association between a bot and a messaging channel. If an alias is used in a channel association, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the channel association that refers to the bot. You can remove the reference to the alias by deleting the channel association. If you get the same exception again, delete the referring association until the <code>DeleteBotAlias</code> operation is successful.</p>
    async fn delete_bot_alias(
        &self,
        input: DeleteBotAliasRequest,
    ) -> Result<(), RusotoError<DeleteBotAliasError>>;

    /// <p>Deletes the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permission for the <code>lex:DeleteBotChannelAssociation</code> action.</p>
    async fn delete_bot_channel_association(
        &self,
        input: DeleteBotChannelAssociationRequest,
    ) -> Result<(), RusotoError<DeleteBotChannelAssociationError>>;

    /// <p>Deletes a specific version of a bot. To delete all versions of a bot, use the <a>DeleteBot</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteBotVersion</code> action.</p>
    async fn delete_bot_version(
        &self,
        input: DeleteBotVersionRequest,
    ) -> Result<(), RusotoError<DeleteBotVersionError>>;

    /// <p>Deletes all versions of the intent, including the <code>$LATEST</code> version. To delete a specific version of the intent, use the <a>DeleteIntentVersion</a> operation.</p> <p> You can delete a version of an intent only if it is not referenced. To delete an intent that is referred to in one or more bots (see <a>how-it-works</a>), you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, it provides an example reference that shows where the intent is referenced. To remove the reference to the intent, either update the bot or delete it. If you get the same exception when you attempt to delete the intent again, repeat until the intent has no references and the call to <code>DeleteIntent</code> is successful. </p> </note> <p> This operation requires permission for the <code>lex:DeleteIntent</code> action. </p>
    async fn delete_intent(
        &self,
        input: DeleteIntentRequest,
    ) -> Result<(), RusotoError<DeleteIntentError>>;

    /// <p>Deletes a specific version of an intent. To delete all versions of a intent, use the <a>DeleteIntent</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteIntentVersion</code> action.</p>
    async fn delete_intent_version(
        &self,
        input: DeleteIntentVersionRequest,
    ) -> Result<(), RusotoError<DeleteIntentVersionError>>;

    /// <p>Deletes all versions of the slot type, including the <code>$LATEST</code> version. To delete a specific version of the slot type, use the <a>DeleteSlotTypeVersion</a> operation.</p> <p> You can delete a version of a slot type only if it is not referenced. To delete a slot type that is referred to in one or more intents, you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, the exception provides an example reference that shows the intent where the slot type is referenced. To remove the reference to the slot type, either update the intent or delete it. If you get the same exception when you attempt to delete the slot type again, repeat until the slot type has no references and the <code>DeleteSlotType</code> call is successful. </p> </note> <p>This operation requires permission for the <code>lex:DeleteSlotType</code> action.</p>
    async fn delete_slot_type(
        &self,
        input: DeleteSlotTypeRequest,
    ) -> Result<(), RusotoError<DeleteSlotTypeError>>;

    /// <p>Deletes a specific version of a slot type. To delete all versions of a slot type, use the <a>DeleteSlotType</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteSlotTypeVersion</code> action.</p>
    async fn delete_slot_type_version(
        &self,
        input: DeleteSlotTypeVersionRequest,
    ) -> Result<(), RusotoError<DeleteSlotTypeVersionError>>;

    /// <p>Deletes stored utterances.</p> <p>Amazon Lex stores the utterances that users send to your bot. Utterances are stored for 15 days for use with the <a>GetUtterancesView</a> operation, and then stored indefinitely for use in improving the ability of your bot to respond to user input.</p> <p>Use the <code>DeleteUtterances</code> operation to manually delete stored utterances for a specific user. When you use the <code>DeleteUtterances</code> operation, utterances stored for improving your bot's ability to respond to user input are deleted immediately. Utterances stored for use with the <code>GetUtterancesView</code> operation are deleted after 15 days.</p> <p>This operation requires permissions for the <code>lex:DeleteUtterances</code> action.</p>
    async fn delete_utterances(
        &self,
        input: DeleteUtterancesRequest,
    ) -> Result<(), RusotoError<DeleteUtterancesError>>;

    /// <p>Returns metadata information for a specific bot. You must provide the bot name and the bot version or alias. </p> <p> This operation requires permissions for the <code>lex:GetBot</code> action. </p>
    async fn get_bot(
        &self,
        input: GetBotRequest,
    ) -> Result<GetBotResponse, RusotoError<GetBotError>>;

    /// <p>Returns information about an Amazon Lex bot alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:GetBotAlias</code> action.</p>
    async fn get_bot_alias(
        &self,
        input: GetBotAliasRequest,
    ) -> Result<GetBotAliasResponse, RusotoError<GetBotAliasError>>;

    /// <p>Returns a list of aliases for a specified Amazon Lex bot.</p> <p>This operation requires permissions for the <code>lex:GetBotAliases</code> action.</p>
    async fn get_bot_aliases(
        &self,
        input: GetBotAliasesRequest,
    ) -> Result<GetBotAliasesResponse, RusotoError<GetBotAliasesError>>;

    /// <p>Returns information about the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permissions for the <code>lex:GetBotChannelAssociation</code> action.</p>
    async fn get_bot_channel_association(
        &self,
        input: GetBotChannelAssociationRequest,
    ) -> Result<GetBotChannelAssociationResponse, RusotoError<GetBotChannelAssociationError>>;

    /// <p> Returns a list of all of the channels associated with the specified bot. </p> <p>The <code>GetBotChannelAssociations</code> operation requires permissions for the <code>lex:GetBotChannelAssociations</code> action.</p>
    async fn get_bot_channel_associations(
        &self,
        input: GetBotChannelAssociationsRequest,
    ) -> Result<GetBotChannelAssociationsResponse, RusotoError<GetBotChannelAssociationsError>>;

    /// <p>Gets information about all of the versions of a bot.</p> <p>The <code>GetBotVersions</code> operation returns a <code>BotMetadata</code> object for each version of a bot. For example, if a bot has three numbered versions, the <code>GetBotVersions</code> operation returns four <code>BotMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetBotVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetBotVersions</code> action.</p>
    async fn get_bot_versions(
        &self,
        input: GetBotVersionsRequest,
    ) -> Result<GetBotVersionsResponse, RusotoError<GetBotVersionsError>>;

    /// <p>Returns bot information as follows: </p> <ul> <li> <p>If you provide the <code>nameContains</code> field, the response includes information for the <code>$LATEST</code> version of all bots whose name contains the specified string.</p> </li> <li> <p>If you don't specify the <code>nameContains</code> field, the operation returns information about the <code>$LATEST</code> version of all of your bots.</p> </li> </ul> <p>This operation requires permission for the <code>lex:GetBots</code> action.</p>
    async fn get_bots(
        &self,
        input: GetBotsRequest,
    ) -> Result<GetBotsResponse, RusotoError<GetBotsError>>;

    /// <p>Returns information about a built-in intent.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntent</code> action.</p>
    async fn get_builtin_intent(
        &self,
        input: GetBuiltinIntentRequest,
    ) -> Result<GetBuiltinIntentResponse, RusotoError<GetBuiltinIntentError>>;

    /// <p>Gets a list of built-in intents that meet the specified criteria.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntents</code> action.</p>
    async fn get_builtin_intents(
        &self,
        input: GetBuiltinIntentsRequest,
    ) -> Result<GetBuiltinIntentsResponse, RusotoError<GetBuiltinIntentsError>>;

    /// <p>Gets a list of built-in slot types that meet the specified criteria.</p> <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p> <p>This operation requires permission for the <code>lex:GetBuiltInSlotTypes</code> action.</p>
    async fn get_builtin_slot_types(
        &self,
        input: GetBuiltinSlotTypesRequest,
    ) -> Result<GetBuiltinSlotTypesResponse, RusotoError<GetBuiltinSlotTypesError>>;

    /// <p>Exports the contents of a Amazon Lex resource in a specified format. </p>
    async fn get_export(
        &self,
        input: GetExportRequest,
    ) -> Result<GetExportResponse, RusotoError<GetExportError>>;

    /// <p>Gets information about an import job started with the <code>StartImport</code> operation.</p>
    async fn get_import(
        &self,
        input: GetImportRequest,
    ) -> Result<GetImportResponse, RusotoError<GetImportError>>;

    /// <p> Returns information about an intent. In addition to the intent name, you must specify the intent version. </p> <p> This operation requires permissions to perform the <code>lex:GetIntent</code> action. </p>
    async fn get_intent(
        &self,
        input: GetIntentRequest,
    ) -> Result<GetIntentResponse, RusotoError<GetIntentError>>;

    /// <p>Gets information about all of the versions of an intent.</p> <p>The <code>GetIntentVersions</code> operation returns an <code>IntentMetadata</code> object for each version of an intent. For example, if an intent has three numbered versions, the <code>GetIntentVersions</code> operation returns four <code>IntentMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetIntentVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetIntentVersions</code> action.</p>
    async fn get_intent_versions(
        &self,
        input: GetIntentVersionsRequest,
    ) -> Result<GetIntentVersionsResponse, RusotoError<GetIntentVersionsError>>;

    /// <p>Returns intent information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all intents that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all intents. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetIntents</code> action. </p>
    async fn get_intents(
        &self,
        input: GetIntentsRequest,
    ) -> Result<GetIntentsResponse, RusotoError<GetIntentsError>>;

    /// <p>Returns information about a specific version of a slot type. In addition to specifying the slot type name, you must specify the slot type version.</p> <p>This operation requires permissions for the <code>lex:GetSlotType</code> action.</p>
    async fn get_slot_type(
        &self,
        input: GetSlotTypeRequest,
    ) -> Result<GetSlotTypeResponse, RusotoError<GetSlotTypeError>>;

    /// <p>Gets information about all versions of a slot type.</p> <p>The <code>GetSlotTypeVersions</code> operation returns a <code>SlotTypeMetadata</code> object for each version of a slot type. For example, if a slot type has three numbered versions, the <code>GetSlotTypeVersions</code> operation returns four <code>SlotTypeMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetSlotTypeVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetSlotTypeVersions</code> action.</p>
    async fn get_slot_type_versions(
        &self,
        input: GetSlotTypeVersionsRequest,
    ) -> Result<GetSlotTypeVersionsResponse, RusotoError<GetSlotTypeVersionsError>>;

    /// <p>Returns slot type information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all slot types that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all slot types. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetSlotTypes</code> action. </p>
    async fn get_slot_types(
        &self,
        input: GetSlotTypesRequest,
    ) -> Result<GetSlotTypesResponse, RusotoError<GetSlotTypesError>>;

    /// <p>Use the <code>GetUtterancesView</code> operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to.</p> <p>For example, say that you have created a bot to order flowers. After your users have used your bot for a while, use the <code>GetUtterancesView</code> operation to see the requests that they have made and whether they have been successful. You might find that the utterance "I want flowers" is not being recognized. You could add this utterance to the <code>OrderFlowers</code> intent so that your bot recognizes that utterance.</p> <p>After you publish a new version of a bot, you can get information about the old version and the new so that you can compare the performance across the two versions. </p> <p>Utterance statistics are generated once a day. Data is available for the last 15 days. You can request information for up to 5 versions of your bot in each request. Amazon Lex returns the most frequent utterances received by the bot in the last 15 days. The response contains information about a maximum of 100 utterances for each version.</p> <p>If you set <code>childDirected</code> field to true when you created your bot, or if you opted out of participating in improving Amazon Lex, utterances are not available.</p> <p>This operation requires permissions for the <code>lex:GetUtterancesView</code> action.</p>
    async fn get_utterances_view(
        &self,
        input: GetUtterancesViewRequest,
    ) -> Result<GetUtterancesViewResponse, RusotoError<GetUtterancesViewError>>;

    /// <p>Creates an Amazon Lex conversational bot or replaces an existing bot. When you create or update a bot you are only required to specify a name, a locale, and whether the bot is directed toward children under age 13. You can use this to add intents later, or to remove intents from an existing bot. When you create a bot with the minimum information, the bot is created or updated but Amazon Lex returns the <code/> response <code>FAILED</code>. You can build the bot after you add one or more intents. For more information about Amazon Lex bots, see <a>how-it-works</a>. </p> <p>If you specify the name of an existing bot, the fields in the request replace the existing values in the <code>$LATEST</code> version of the bot. Amazon Lex removes any fields that you don't provide values for in the request, except for the <code>idleTTLInSeconds</code> and <code>privacySettings</code> fields, which are set to their default values. If you don't specify values for required fields, Amazon Lex throws an exception.</p> <p>This operation requires permissions for the <code>lex:PutBot</code> action. For more information, see <a>security-iam</a>.</p>
    async fn put_bot(
        &self,
        input: PutBotRequest,
    ) -> Result<PutBotResponse, RusotoError<PutBotError>>;

    /// <p>Creates an alias for the specified version of the bot or replaces an alias for the specified bot. To change the version of the bot that the alias points to, replace the alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:PutBotAlias</code> action. </p>
    async fn put_bot_alias(
        &self,
        input: PutBotAliasRequest,
    ) -> Result<PutBotAliasResponse, RusotoError<PutBotAliasError>>;

    /// <p>Creates an intent or replaces an existing intent.</p> <p>To define the interaction between the user and your bot, you use one or more intents. For a pizza ordering bot, for example, you would create an <code>OrderPizza</code> intent. </p> <p>To create an intent or replace an existing intent, you must provide the following:</p> <ul> <li> <p>Intent name. For example, <code>OrderPizza</code>.</p> </li> <li> <p>Sample utterances. For example, "Can I order a pizza, please." and "I want to order a pizza."</p> </li> <li> <p>Information to be gathered. You specify slot types for the information that your bot will request from the user. You can specify standard slot types, such as a date or a time, or custom slot types such as the size and crust of a pizza.</p> </li> <li> <p>How the intent will be fulfilled. You can provide a Lambda function or configure the intent to return the intent information to the client application. If you use a Lambda function, when all of the intent information is available, Amazon Lex invokes your Lambda function. If you configure your intent to return the intent information to the client application. </p> </li> </ul> <p>You can specify other optional information in the request, such as:</p> <ul> <li> <p>A confirmation prompt to ask the user to confirm an intent. For example, "Shall I order your pizza?"</p> </li> <li> <p>A conclusion statement to send to the user after the intent has been fulfilled. For example, "I placed your pizza order."</p> </li> <li> <p>A follow-up prompt that asks the user for additional activity. For example, asking "Do you want to order a drink with your pizza?"</p> </li> </ul> <p>If you specify an existing intent name to update the intent, Amazon Lex replaces the values in the <code>$LATEST</code> version of the intent with the values in the request. Amazon Lex removes fields that you don't provide in the request. If you don't specify the required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of an intent, the <code>status</code> field of any bot that uses the <code>$LATEST</code> version of the intent is set to <code>NOT_BUILT</code>.</p> <p>For more information, see <a>how-it-works</a>.</p> <p>This operation requires permissions for the <code>lex:PutIntent</code> action.</p>
    async fn put_intent(
        &self,
        input: PutIntentRequest,
    ) -> Result<PutIntentResponse, RusotoError<PutIntentError>>;

    /// <p>Creates a custom slot type or replaces an existing custom slot type.</p> <p>To create a custom slot type, specify a name for the slot type and a set of enumeration values, which are the values that a slot of this type can assume. For more information, see <a>how-it-works</a>.</p> <p>If you specify the name of an existing slot type, the fields in the request replace the existing values in the <code>$LATEST</code> version of the slot type. Amazon Lex removes the fields that you don't provide in the request. If you don't specify required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of a slot type, if a bot uses the <code>$LATEST</code> version of an intent that contains the slot type, the bot's <code>status</code> field is set to <code>NOT_BUILT</code>.</p> <p>This operation requires permissions for the <code>lex:PutSlotType</code> action.</p>
    async fn put_slot_type(
        &self,
        input: PutSlotTypeRequest,
    ) -> Result<PutSlotTypeResponse, RusotoError<PutSlotTypeError>>;

    /// <p>Starts a job to import a resource to Amazon Lex.</p>
    async fn start_import(
        &self,
        input: StartImportRequest,
    ) -> Result<StartImportResponse, RusotoError<StartImportError>>;
}
/// A client for the Amazon Lex Model Building Service API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LexModelsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LexModelsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LexModelsClient {
        LexModelsClient { client, region }
    }
}

#[async_trait]
impl LexModels for LexModelsClient {
    /// <p>Creates a new version of the bot based on the <code>$LATEST</code> version. If the <code>$LATEST</code> version of this resource hasn't changed since you created the last version, Amazon Lex doesn't create a new version. It returns the last created version.</p> <note> <p>You can update only the <code>$LATEST</code> version of the bot. You can't update the numbered versions that you create with the <code>CreateBotVersion</code> operation.</p> </note> <p> When you create the first version of a bot, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p> This operation requires permission for the <code>lex:CreateBotVersion</code> action. </p>
    async fn create_bot_version(
        &self,
        input: CreateBotVersionRequest,
    ) -> Result<CreateBotVersionResponse, RusotoError<CreateBotVersionError>> {
        let request_uri = format!("/bots/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateBotVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateBotVersionError::from_response(response))
        }
    }

    /// <p>Creates a new version of an intent based on the <code>$LATEST</code> version of the intent. If the <code>$LATEST</code> version of this intent hasn't changed since you last updated it, Amazon Lex doesn't create a new version. It returns the last version you created.</p> <note> <p>You can update only the <code>$LATEST</code> version of the intent. You can't update the numbered versions that you create with the <code>CreateIntentVersion</code> operation.</p> </note> <p> When you create a version of an intent, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions to perform the <code>lex:CreateIntentVersion</code> action. </p>
    async fn create_intent_version(
        &self,
        input: CreateIntentVersionRequest,
    ) -> Result<CreateIntentVersionResponse, RusotoError<CreateIntentVersionError>> {
        let request_uri = format!("/intents/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateIntentVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateIntentVersionError::from_response(response))
        }
    }

    /// <p>Creates a new version of a slot type based on the <code>$LATEST</code> version of the specified slot type. If the <code>$LATEST</code> version of this resource has not changed since the last version that you created, Amazon Lex doesn't create a new version. It returns the last version that you created. </p> <note> <p>You can update only the <code>$LATEST</code> version of a slot type. You can't update the numbered versions that you create with the <code>CreateSlotTypeVersion</code> operation.</p> </note> <p>When you create a version of a slot type, Amazon Lex sets the version to 1. Subsequent versions increment by 1. For more information, see <a>versioning-intro</a>. </p> <p>This operation requires permissions for the <code>lex:CreateSlotTypeVersion</code> action.</p>
    async fn create_slot_type_version(
        &self,
        input: CreateSlotTypeVersionRequest,
    ) -> Result<CreateSlotTypeVersionResponse, RusotoError<CreateSlotTypeVersionError>> {
        let request_uri = format!("/slottypes/{name}/versions", name = input.name);

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSlotTypeVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSlotTypeVersionError::from_response(response))
        }
    }

    /// <p>Deletes all versions of the bot, including the <code>$LATEST</code> version. To delete a specific version of the bot, use the <a>DeleteBotVersion</a> operation. The <code>DeleteBot</code> operation doesn't immediately remove the bot schema. Instead, it is marked for deletion and removed later.</p> <p>Amazon Lex stores utterances indefinitely for improving the ability of your bot to respond to user inputs. These utterances are not removed when the bot is deleted. To remove the utterances, use the <a>DeleteUtterances</a> operation.</p> <p>If a bot has an alias, you can't delete it. Instead, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the alias that refers to the bot. To remove the reference to the bot, delete the alias. If you get the same exception again, delete the referring alias until the <code>DeleteBot</code> operation is successful.</p> <p>This operation requires permissions for the <code>lex:DeleteBot</code> action.</p>
    async fn delete_bot(&self, input: DeleteBotRequest) -> Result<(), RusotoError<DeleteBotError>> {
        let request_uri = format!("/bots/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBotError::from_response(response))
        }
    }

    /// <p>Deletes an alias for the specified bot. </p> <p>You can't delete an alias that is used in the association between a bot and a messaging channel. If an alias is used in a channel association, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the channel association that refers to the bot. You can remove the reference to the alias by deleting the channel association. If you get the same exception again, delete the referring association until the <code>DeleteBotAlias</code> operation is successful.</p>
    async fn delete_bot_alias(
        &self,
        input: DeleteBotAliasRequest,
    ) -> Result<(), RusotoError<DeleteBotAliasError>> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{name}",
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBotAliasError::from_response(response))
        }
    }

    /// <p>Deletes the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permission for the <code>lex:DeleteBotChannelAssociation</code> action.</p>
    async fn delete_bot_channel_association(
        &self,
        input: DeleteBotChannelAssociationRequest,
    ) -> Result<(), RusotoError<DeleteBotChannelAssociationError>> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{alias_name}/channels/{name}",
            alias_name = input.bot_alias,
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBotChannelAssociationError::from_response(response))
        }
    }

    /// <p>Deletes a specific version of a bot. To delete all versions of a bot, use the <a>DeleteBot</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteBotVersion</code> action.</p>
    async fn delete_bot_version(
        &self,
        input: DeleteBotVersionRequest,
    ) -> Result<(), RusotoError<DeleteBotVersionError>> {
        let request_uri = format!(
            "/bots/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBotVersionError::from_response(response))
        }
    }

    /// <p>Deletes all versions of the intent, including the <code>$LATEST</code> version. To delete a specific version of the intent, use the <a>DeleteIntentVersion</a> operation.</p> <p> You can delete a version of an intent only if it is not referenced. To delete an intent that is referred to in one or more bots (see <a>how-it-works</a>), you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, it provides an example reference that shows where the intent is referenced. To remove the reference to the intent, either update the bot or delete it. If you get the same exception when you attempt to delete the intent again, repeat until the intent has no references and the call to <code>DeleteIntent</code> is successful. </p> </note> <p> This operation requires permission for the <code>lex:DeleteIntent</code> action. </p>
    async fn delete_intent(
        &self,
        input: DeleteIntentRequest,
    ) -> Result<(), RusotoError<DeleteIntentError>> {
        let request_uri = format!("/intents/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIntentError::from_response(response))
        }
    }

    /// <p>Deletes a specific version of an intent. To delete all versions of a intent, use the <a>DeleteIntent</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteIntentVersion</code> action.</p>
    async fn delete_intent_version(
        &self,
        input: DeleteIntentVersionRequest,
    ) -> Result<(), RusotoError<DeleteIntentVersionError>> {
        let request_uri = format!(
            "/intents/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteIntentVersionError::from_response(response))
        }
    }

    /// <p>Deletes all versions of the slot type, including the <code>$LATEST</code> version. To delete a specific version of the slot type, use the <a>DeleteSlotTypeVersion</a> operation.</p> <p> You can delete a version of a slot type only if it is not referenced. To delete a slot type that is referred to in one or more intents, you must remove those references first. </p> <note> <p> If you get the <code>ResourceInUseException</code> exception, the exception provides an example reference that shows the intent where the slot type is referenced. To remove the reference to the slot type, either update the intent or delete it. If you get the same exception when you attempt to delete the slot type again, repeat until the slot type has no references and the <code>DeleteSlotType</code> call is successful. </p> </note> <p>This operation requires permission for the <code>lex:DeleteSlotType</code> action.</p>
    async fn delete_slot_type(
        &self,
        input: DeleteSlotTypeRequest,
    ) -> Result<(), RusotoError<DeleteSlotTypeError>> {
        let request_uri = format!("/slottypes/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSlotTypeError::from_response(response))
        }
    }

    /// <p>Deletes a specific version of a slot type. To delete all versions of a slot type, use the <a>DeleteSlotType</a> operation. </p> <p>This operation requires permissions for the <code>lex:DeleteSlotTypeVersion</code> action.</p>
    async fn delete_slot_type_version(
        &self,
        input: DeleteSlotTypeVersionRequest,
    ) -> Result<(), RusotoError<DeleteSlotTypeVersionError>> {
        let request_uri = format!(
            "/slottypes/{name}/version/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSlotTypeVersionError::from_response(response))
        }
    }

    /// <p>Deletes stored utterances.</p> <p>Amazon Lex stores the utterances that users send to your bot. Utterances are stored for 15 days for use with the <a>GetUtterancesView</a> operation, and then stored indefinitely for use in improving the ability of your bot to respond to user input.</p> <p>Use the <code>DeleteUtterances</code> operation to manually delete stored utterances for a specific user. When you use the <code>DeleteUtterances</code> operation, utterances stored for improving your bot's ability to respond to user input are deleted immediately. Utterances stored for use with the <code>GetUtterancesView</code> operation are deleted after 15 days.</p> <p>This operation requires permissions for the <code>lex:DeleteUtterances</code> action.</p>
    async fn delete_utterances(
        &self,
        input: DeleteUtterancesRequest,
    ) -> Result<(), RusotoError<DeleteUtterancesError>> {
        let request_uri = format!(
            "/bots/{bot_name}/utterances/{user_id}",
            bot_name = input.bot_name,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("DELETE", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUtterancesError::from_response(response))
        }
    }

    /// <p>Returns metadata information for a specific bot. You must provide the bot name and the bot version or alias. </p> <p> This operation requires permissions for the <code>lex:GetBot</code> action. </p>
    async fn get_bot(
        &self,
        input: GetBotRequest,
    ) -> Result<GetBotResponse, RusotoError<GetBotError>> {
        let request_uri = format!(
            "/bots/{name}/versions/{versionoralias}",
            name = input.name,
            versionoralias = input.version_or_alias
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetBotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotError::from_response(response))
        }
    }

    /// <p>Returns information about an Amazon Lex bot alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:GetBotAlias</code> action.</p>
    async fn get_bot_alias(
        &self,
        input: GetBotAliasRequest,
    ) -> Result<GetBotAliasResponse, RusotoError<GetBotAliasError>> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{name}",
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBotAliasResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotAliasError::from_response(response))
        }
    }

    /// <p>Returns a list of aliases for a specified Amazon Lex bot.</p> <p>This operation requires permissions for the <code>lex:GetBotAliases</code> action.</p>
    async fn get_bot_aliases(
        &self,
        input: GetBotAliasesRequest,
    ) -> Result<GetBotAliasesResponse, RusotoError<GetBotAliasesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBotAliasesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotAliasesError::from_response(response))
        }
    }

    /// <p>Returns information about the association between an Amazon Lex bot and a messaging platform.</p> <p>This operation requires permissions for the <code>lex:GetBotChannelAssociation</code> action.</p>
    async fn get_bot_channel_association(
        &self,
        input: GetBotChannelAssociationRequest,
    ) -> Result<GetBotChannelAssociationResponse, RusotoError<GetBotChannelAssociationError>> {
        let request_uri = format!(
            "/bots/{bot_name}/aliases/{alias_name}/channels/{name}",
            alias_name = input.bot_alias,
            bot_name = input.bot_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBotChannelAssociationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotChannelAssociationError::from_response(response))
        }
    }

    /// <p> Returns a list of all of the channels associated with the specified bot. </p> <p>The <code>GetBotChannelAssociations</code> operation requires permissions for the <code>lex:GetBotChannelAssociations</code> action.</p>
    async fn get_bot_channel_associations(
        &self,
        input: GetBotChannelAssociationsRequest,
    ) -> Result<GetBotChannelAssociationsResponse, RusotoError<GetBotChannelAssociationsError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBotChannelAssociationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotChannelAssociationsError::from_response(response))
        }
    }

    /// <p>Gets information about all of the versions of a bot.</p> <p>The <code>GetBotVersions</code> operation returns a <code>BotMetadata</code> object for each version of a bot. For example, if a bot has three numbered versions, the <code>GetBotVersions</code> operation returns four <code>BotMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetBotVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetBotVersions</code> action.</p>
    async fn get_bot_versions(
        &self,
        input: GetBotVersionsRequest,
    ) -> Result<GetBotVersionsResponse, RusotoError<GetBotVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBotVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotVersionsError::from_response(response))
        }
    }

    /// <p>Returns bot information as follows: </p> <ul> <li> <p>If you provide the <code>nameContains</code> field, the response includes information for the <code>$LATEST</code> version of all bots whose name contains the specified string.</p> </li> <li> <p>If you don't specify the <code>nameContains</code> field, the operation returns information about the <code>$LATEST</code> version of all of your bots.</p> </li> </ul> <p>This operation requires permission for the <code>lex:GetBots</code> action.</p>
    async fn get_bots(
        &self,
        input: GetBotsRequest,
    ) -> Result<GetBotsResponse, RusotoError<GetBotsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetBotsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBotsError::from_response(response))
        }
    }

    /// <p>Returns information about a built-in intent.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntent</code> action.</p>
    async fn get_builtin_intent(
        &self,
        input: GetBuiltinIntentRequest,
    ) -> Result<GetBuiltinIntentResponse, RusotoError<GetBuiltinIntentError>> {
        let request_uri = format!("/builtins/intents/{signature}", signature = input.signature);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBuiltinIntentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBuiltinIntentError::from_response(response))
        }
    }

    /// <p>Gets a list of built-in intents that meet the specified criteria.</p> <p>This operation requires permission for the <code>lex:GetBuiltinIntents</code> action.</p>
    async fn get_builtin_intents(
        &self,
        input: GetBuiltinIntentsRequest,
    ) -> Result<GetBuiltinIntentsResponse, RusotoError<GetBuiltinIntentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBuiltinIntentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBuiltinIntentsError::from_response(response))
        }
    }

    /// <p>Gets a list of built-in slot types that meet the specified criteria.</p> <p>For a list of built-in slot types, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/slot-type-reference">Slot Type Reference</a> in the <i>Alexa Skills Kit</i>.</p> <p>This operation requires permission for the <code>lex:GetBuiltInSlotTypes</code> action.</p>
    async fn get_builtin_slot_types(
        &self,
        input: GetBuiltinSlotTypesRequest,
    ) -> Result<GetBuiltinSlotTypesResponse, RusotoError<GetBuiltinSlotTypesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBuiltinSlotTypesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBuiltinSlotTypesError::from_response(response))
        }
    }

    /// <p>Exports the contents of a Amazon Lex resource in a specified format. </p>
    async fn get_export(
        &self,
        input: GetExportRequest,
    ) -> Result<GetExportResponse, RusotoError<GetExportError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetExportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetExportError::from_response(response))
        }
    }

    /// <p>Gets information about an import job started with the <code>StartImport</code> operation.</p>
    async fn get_import(
        &self,
        input: GetImportRequest,
    ) -> Result<GetImportResponse, RusotoError<GetImportError>> {
        let request_uri = format!("/imports/{import_id}", import_id = input.import_id);

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImportError::from_response(response))
        }
    }

    /// <p> Returns information about an intent. In addition to the intent name, you must specify the intent version. </p> <p> This operation requires permissions to perform the <code>lex:GetIntent</code> action. </p>
    async fn get_intent(
        &self,
        input: GetIntentRequest,
    ) -> Result<GetIntentResponse, RusotoError<GetIntentError>> {
        let request_uri = format!(
            "/intents/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntentError::from_response(response))
        }
    }

    /// <p>Gets information about all of the versions of an intent.</p> <p>The <code>GetIntentVersions</code> operation returns an <code>IntentMetadata</code> object for each version of an intent. For example, if an intent has three numbered versions, the <code>GetIntentVersions</code> operation returns four <code>IntentMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetIntentVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetIntentVersions</code> action.</p>
    async fn get_intent_versions(
        &self,
        input: GetIntentVersionsRequest,
    ) -> Result<GetIntentVersionsResponse, RusotoError<GetIntentVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntentVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntentVersionsError::from_response(response))
        }
    }

    /// <p>Returns intent information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all intents that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all intents. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetIntents</code> action. </p>
    async fn get_intents(
        &self,
        input: GetIntentsRequest,
    ) -> Result<GetIntentsResponse, RusotoError<GetIntentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIntentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIntentsError::from_response(response))
        }
    }

    /// <p>Returns information about a specific version of a slot type. In addition to specifying the slot type name, you must specify the slot type version.</p> <p>This operation requires permissions for the <code>lex:GetSlotType</code> action.</p>
    async fn get_slot_type(
        &self,
        input: GetSlotTypeRequest,
    ) -> Result<GetSlotTypeResponse, RusotoError<GetSlotTypeError>> {
        let request_uri = format!(
            "/slottypes/{name}/versions/{version}",
            name = input.name,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSlotTypeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSlotTypeError::from_response(response))
        }
    }

    /// <p>Gets information about all versions of a slot type.</p> <p>The <code>GetSlotTypeVersions</code> operation returns a <code>SlotTypeMetadata</code> object for each version of a slot type. For example, if a slot type has three numbered versions, the <code>GetSlotTypeVersions</code> operation returns four <code>SlotTypeMetadata</code> objects in the response, one for each numbered version and one for the <code>$LATEST</code> version. </p> <p>The <code>GetSlotTypeVersions</code> operation always returns at least one version, the <code>$LATEST</code> version.</p> <p>This operation requires permissions for the <code>lex:GetSlotTypeVersions</code> action.</p>
    async fn get_slot_type_versions(
        &self,
        input: GetSlotTypeVersionsRequest,
    ) -> Result<GetSlotTypeVersionsResponse, RusotoError<GetSlotTypeVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSlotTypeVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSlotTypeVersionsError::from_response(response))
        }
    }

    /// <p>Returns slot type information as follows: </p> <ul> <li> <p>If you specify the <code>nameContains</code> field, returns the <code>$LATEST</code> version of all slot types that contain the specified string.</p> </li> <li> <p> If you don't specify the <code>nameContains</code> field, returns information about the <code>$LATEST</code> version of all slot types. </p> </li> </ul> <p> The operation requires permission for the <code>lex:GetSlotTypes</code> action. </p>
    async fn get_slot_types(
        &self,
        input: GetSlotTypesRequest,
    ) -> Result<GetSlotTypesResponse, RusotoError<GetSlotTypesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSlotTypesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSlotTypesError::from_response(response))
        }
    }

    /// <p>Use the <code>GetUtterancesView</code> operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to.</p> <p>For example, say that you have created a bot to order flowers. After your users have used your bot for a while, use the <code>GetUtterancesView</code> operation to see the requests that they have made and whether they have been successful. You might find that the utterance "I want flowers" is not being recognized. You could add this utterance to the <code>OrderFlowers</code> intent so that your bot recognizes that utterance.</p> <p>After you publish a new version of a bot, you can get information about the old version and the new so that you can compare the performance across the two versions. </p> <p>Utterance statistics are generated once a day. Data is available for the last 15 days. You can request information for up to 5 versions of your bot in each request. Amazon Lex returns the most frequent utterances received by the bot in the last 15 days. The response contains information about a maximum of 100 utterances for each version.</p> <p>If you set <code>childDirected</code> field to true when you created your bot, or if you opted out of participating in improving Amazon Lex, utterances are not available.</p> <p>This operation requires permissions for the <code>lex:GetUtterancesView</code> action.</p>
    async fn get_utterances_view(
        &self,
        input: GetUtterancesViewRequest,
    ) -> Result<GetUtterancesViewResponse, RusotoError<GetUtterancesViewError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUtterancesViewResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUtterancesViewError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Lex conversational bot or replaces an existing bot. When you create or update a bot you are only required to specify a name, a locale, and whether the bot is directed toward children under age 13. You can use this to add intents later, or to remove intents from an existing bot. When you create a bot with the minimum information, the bot is created or updated but Amazon Lex returns the <code/> response <code>FAILED</code>. You can build the bot after you add one or more intents. For more information about Amazon Lex bots, see <a>how-it-works</a>. </p> <p>If you specify the name of an existing bot, the fields in the request replace the existing values in the <code>$LATEST</code> version of the bot. Amazon Lex removes any fields that you don't provide values for in the request, except for the <code>idleTTLInSeconds</code> and <code>privacySettings</code> fields, which are set to their default values. If you don't specify values for required fields, Amazon Lex throws an exception.</p> <p>This operation requires permissions for the <code>lex:PutBot</code> action. For more information, see <a>security-iam</a>.</p>
    async fn put_bot(
        &self,
        input: PutBotRequest,
    ) -> Result<PutBotResponse, RusotoError<PutBotError>> {
        let request_uri = format!("/bots/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<PutBotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutBotError::from_response(response))
        }
    }

    /// <p>Creates an alias for the specified version of the bot or replaces an alias for the specified bot. To change the version of the bot that the alias points to, replace the alias. For more information about aliases, see <a>versioning-aliases</a>.</p> <p>This operation requires permissions for the <code>lex:PutBotAlias</code> action. </p>
    async fn put_bot_alias(
        &self,
        input: PutBotAliasRequest,
    ) -> Result<PutBotAliasResponse, RusotoError<PutBotAliasError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutBotAliasResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutBotAliasError::from_response(response))
        }
    }

    /// <p>Creates an intent or replaces an existing intent.</p> <p>To define the interaction between the user and your bot, you use one or more intents. For a pizza ordering bot, for example, you would create an <code>OrderPizza</code> intent. </p> <p>To create an intent or replace an existing intent, you must provide the following:</p> <ul> <li> <p>Intent name. For example, <code>OrderPizza</code>.</p> </li> <li> <p>Sample utterances. For example, "Can I order a pizza, please." and "I want to order a pizza."</p> </li> <li> <p>Information to be gathered. You specify slot types for the information that your bot will request from the user. You can specify standard slot types, such as a date or a time, or custom slot types such as the size and crust of a pizza.</p> </li> <li> <p>How the intent will be fulfilled. You can provide a Lambda function or configure the intent to return the intent information to the client application. If you use a Lambda function, when all of the intent information is available, Amazon Lex invokes your Lambda function. If you configure your intent to return the intent information to the client application. </p> </li> </ul> <p>You can specify other optional information in the request, such as:</p> <ul> <li> <p>A confirmation prompt to ask the user to confirm an intent. For example, "Shall I order your pizza?"</p> </li> <li> <p>A conclusion statement to send to the user after the intent has been fulfilled. For example, "I placed your pizza order."</p> </li> <li> <p>A follow-up prompt that asks the user for additional activity. For example, asking "Do you want to order a drink with your pizza?"</p> </li> </ul> <p>If you specify an existing intent name to update the intent, Amazon Lex replaces the values in the <code>$LATEST</code> version of the intent with the values in the request. Amazon Lex removes fields that you don't provide in the request. If you don't specify the required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of an intent, the <code>status</code> field of any bot that uses the <code>$LATEST</code> version of the intent is set to <code>NOT_BUILT</code>.</p> <p>For more information, see <a>how-it-works</a>.</p> <p>This operation requires permissions for the <code>lex:PutIntent</code> action.</p>
    async fn put_intent(
        &self,
        input: PutIntentRequest,
    ) -> Result<PutIntentResponse, RusotoError<PutIntentError>> {
        let request_uri = format!("/intents/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutIntentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutIntentError::from_response(response))
        }
    }

    /// <p>Creates a custom slot type or replaces an existing custom slot type.</p> <p>To create a custom slot type, specify a name for the slot type and a set of enumeration values, which are the values that a slot of this type can assume. For more information, see <a>how-it-works</a>.</p> <p>If you specify the name of an existing slot type, the fields in the request replace the existing values in the <code>$LATEST</code> version of the slot type. Amazon Lex removes the fields that you don't provide in the request. If you don't specify required fields, Amazon Lex throws an exception. When you update the <code>$LATEST</code> version of a slot type, if a bot uses the <code>$LATEST</code> version of an intent that contains the slot type, the bot's <code>status</code> field is set to <code>NOT_BUILT</code>.</p> <p>This operation requires permissions for the <code>lex:PutSlotType</code> action.</p>
    async fn put_slot_type(
        &self,
        input: PutSlotTypeRequest,
    ) -> Result<PutSlotTypeResponse, RusotoError<PutSlotTypeError>> {
        let request_uri = format!("/slottypes/{name}/versions/$LATEST", name = input.name);

        let mut request = SignedRequest::new("PUT", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutSlotTypeResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSlotTypeError::from_response(response))
        }
    }

    /// <p>Starts a job to import a resource to Amazon Lex.</p>
    async fn start_import(
        &self,
        input: StartImportRequest,
    ) -> Result<StartImportResponse, RusotoError<StartImportError>> {
        let request_uri = "/imports/";

        let mut request = SignedRequest::new("POST", "lex", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("models.lex".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartImportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartImportError::from_response(response))
        }
    }
}
