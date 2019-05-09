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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Amazon Device Messaging channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ADMChannelRequest {
    /// <p>The Client ID that you obtained from the Amazon App Distribution Portal.</p>
    #[serde(rename = "ClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The Client Secret that you obtained from the Amazon App Distribution Portal.</p>
    #[serde(rename = "ClientSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// <p>Indicates whether or not the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Amazon Device Messaging channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ADMChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when this channel was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Indicates whether or not the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the channel. Retained for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether or not the channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last updated this channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when this channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. For this channel, the value is always &quot;ADM.&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The channel version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>ADM Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ADMMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Optional. Arbitrary string used to indicate multiple messages are logically the same and that ADM is allowed to drop previously enqueued messages in favor of this one.</p>
    #[serde(rename = "ConsolidationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidation_key: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>Optional. Number of seconds ADM should retain the message if the device is offline</p>
    #[serde(rename = "ExpiresAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<String>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>Optional. Base-64-encoded MD5 checksum of the data parameter. Used to verify data integrity</p>
    #[serde(rename = "MD5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Apple Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple Distribution Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSChannelResponse {
    /// <p>The ID of the application that the channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when this channel was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Indicates whether the channel is configured with a key for APNs token authentication. Provide a token key by setting the TokenKey attribute.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>(Deprecated) An identifier for the channel. Retained for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether or not the channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last updated this channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when this channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. For this channel, the value is always &quot;ADM.&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The channel version.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>APNS Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>Include this key when you want the system to modify the badge of your app icon. If this key is not included in the dictionary, the badge is not changed. To remove the badge, set the value of this key to 0.</p>
    #[serde(rename = "Badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<i64>,
    /// <p>The message body of the notification.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Provide this key with a string value that represents the notification&#39;s type. This value corresponds to the value in the identifier property of one of your app&#39;s registered categories.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>An ID that, if assigned to multiple messages, causes APNs to coalesce the messages into a single push notification instead of delivering each message individually. The value must not exceed 64 bytes. Amazon Pinpoint uses this value to set the apns-collapse-id request header when it sends the message to APNs.</p>
    #[serde(rename = "CollapseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>A URL that refers to the location of an image or video that you want to display in the push notification.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The preferred authentication method, either &quot;CERTIFICATE&quot; or &quot;TOKEN&quot;</p>
    #[serde(rename = "PreferredAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_authentication_method: Option<String>,
    /// <p>The message priority. Amazon Pinpoint uses this value to set the apns-priority request header when it sends the message to APNs. Accepts the following values:</p>
    ///
    /// <p>&quot;5&quot; - Low priority. Messages might be delayed, delivered in groups, and throttled.</p>
    ///
    /// <p>&quot;10&quot; - High priority. Messages are sent immediately. High priority messages must cause an alert, sound, or badge on the receiving device.</p>
    ///
    /// <p>The default value is &quot;10&quot;.</p>
    ///
    /// <p>The equivalent values for FCM or GCM messages are &quot;normal&quot; and &quot;high&quot;. Amazon Pinpoint accepts these values for APNs messages and converts them.</p>
    ///
    /// <p>For more information about the apns-priority parameter, see Communicating with APNs in the APNs Local and Remote Notification Programming Guide.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>Include this key when you want the system to play a sound. The value of this key is the name of a sound file in your app&#39;s main bundle or in the Library/Sounds folder of your app&#39;s data container. If the sound file cannot be found, or if you specify defaultfor the value, the system plays the default alert sound.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Provide this key with a string value that represents the app-specific identifier for grouping notifications. If you provide a Notification Content app extension, you can use this value to group your notifications together.</p>
    #[serde(rename = "ThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// <p>The length of time (in seconds) that APNs stores and attempts to deliver the message. If the value is 0, APNs does not store the message or attempt to deliver it more than once. Amazon Pinpoint uses this value to set the apns-expiration request header when it sends the message to APNs.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Apple Development Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSSandboxChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple Development Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSSandboxChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Indicates whether the channel is configured with a key for APNs token authentication. Provide a token key by setting the TokenKey attribute.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS_SANDBOX.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Apple VoIP Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSVoipChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple VoIP Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSVoipChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>If the channel is registered with a token key for authentication.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Apple VoIP Developer Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct APNSVoipSandboxChannelRequest {
    /// <p>The bundle id used for APNs Tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The distribution certificate from Apple.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The certificate private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The team id used for APNs Tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The token key used for APNs Tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Apple VoIP Developer Push Notification Service channel definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct APNSVoipSandboxChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method used for APNs.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>If the channel is registered with a token key for authentication.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be APNS.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Activities for campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActivitiesResponse {
    /// <p>List of campaign activities</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ActivityResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Activity definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ActivityResponse {
    /// <p>The ID of the application to which the campaign applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The ID of the campaign to which the activity applies.</p>
    #[serde(rename = "CampaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// <p>The actual time the activity was marked CANCELLED or COMPLETED. Provided in ISO 8601 format.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The unique activity ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether the activity succeeded.</p>
    ///
    /// <p>Valid values: SUCCESS, FAIL</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The scheduled start time for the activity in ISO 8601 format.</p>
    #[serde(rename = "ScheduledStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<String>,
    /// <p>The actual start time of the activity in ISO 8601 format.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>The state of the activity.</p>
    ///
    /// <p>Valid values: PENDING, INITIALIZING, RUNNING, PAUSED, CANCELLED, COMPLETED</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The total number of endpoints to which the campaign successfully delivered messages.</p>
    #[serde(rename = "SuccessfulEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_endpoint_count: Option<i64>,
    /// <p>The total number of timezones completed.</p>
    #[serde(rename = "TimezonesCompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_completed_count: Option<i64>,
    /// <p>The total number of unique timezones present in the segment.</p>
    #[serde(rename = "TimezonesTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_total_count: Option<i64>,
    /// <p>The total number of endpoints to which the campaign attempts to deliver messages.</p>
    #[serde(rename = "TotalEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_endpoint_count: Option<i64>,
    /// <p>The ID of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_id: Option<String>,
}

/// <p>Address configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddressConfiguration {
    /// <p>Body override. If specified will override default body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A map of custom attributes to attributes to be attached to the message for this address. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of substitution values for the message to be merged with the DefaultMessage&#39;s substitutions. Substitutions on this map take precedence over the all other substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Title override. If specified will override default title if applicable.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Application Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationResponse {
    /// <p>The unique application ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The display name of the application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Application settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationSettingsResource {
    /// <p>The unique ID for the application.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>Default campaign hook.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The default campaign limits for the app. These limits apply to each campaign for the app, unless the campaign overrides the default with limits of its own.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for the app. Campaigns in the app don&#39;t send messages to endpoints during the quiet time.</p>
    ///
    /// <p>Note: Make sure that your endpoints include the Demographics.Timezone attribute if you plan to enable a quiet time for your app. If your endpoints don&#39;t include this attribute, they&#39;ll receive the messages that you send them, even if quiet time is enabled.</p>
    ///
    /// <p>When you set up an app to use quiet time, campaigns in that app don&#39;t send messages during the time range you specified, as long as all of the following are true:
    /// - The endpoint includes a valid Demographic.Timezone attribute.
    /// - The current time in the endpoint&#39;s time zone is later than or equal to the time specified in the QuietTime.Start attribute for the app (or campaign, if applicable).
    /// - The current time in the endpoint&#39;s time zone is earlier than or equal to the time specified in the QuietTime.End attribute for the app (or campaign, if applicable).</p>
    ///
    /// <p>Individual campaigns within the app can have their own quiet time settings, which override the quiet time settings at the app level.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Get Applications Result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplicationsResponse {
    /// <p>List of applications returned in this page.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ApplicationResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Custom attibute dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeDimension {
    /// <p>The type of dimension:</p>
    ///
    /// <p>INCLUSIVE - Endpoints that match the criteria are included in the segment.</p>
    ///
    /// <p>EXCLUSIVE - Endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttributesResource {
    /// <p>The unique ID for the application.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The attribute type for the application.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The attributes for the application.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

/// <p>Baidu Cloud Push credentials</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BaiduChannelRequest {
    /// <p>Platform credential API key from Baidu.</p>
    #[serde(rename = "ApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Platform credential Secret key from Baidu.</p>
    #[serde(rename = "SecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

/// <p>Baidu Cloud Messaging channel definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BaiduChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The Baidu API key from Baidu.</p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be BAIDU</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Baidu Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BaiduMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>This parameter specifies how long (in seconds) the message should be kept in Baidu storage if the device is offline. The and the default value and the maximum time to live supported is 7 days (604800 seconds)</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>The email message configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignEmailMessage {
    /// <p>The email text body.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The email address used to send the email from. Defaults to use FromAddress specified in the Email Channel.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The email html body.</p>
    #[serde(rename = "HtmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    /// <p>The email title (Or subject).</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>An object that defines the events that cause the campaign to be sent.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignEventFilter {
    /// <p>An object that defines the dimensions for the event filter.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<EventDimensions>,
    /// <p>The type of event that causes the campaign to be sent. Possible values:</p>
    ///
    /// <p>SYSTEM - Send the campaign when a system event occurs. See the System resource for more information.</p>
    ///
    /// <p>ENDPOINT - Send the campaign when an endpoint event occurs. See the Event resource for more information.</p>
    #[serde(rename = "FilterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
}

/// <p>Campaign hook information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignHook {
    /// <p>Lambda function name or arn to be called for delivery</p>
    #[serde(rename = "LambdaFunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<String>,
    /// <p>What mode Lambda should be invoked in.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Web URL to call for hook. If the URL has authentication specified it will be added as authentication to the request</p>
    #[serde(rename = "WebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// <p>Campaign Limits are used to limit the number of messages that can be sent to a single endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignLimits {
    /// <p>The maximum number of messages that each campaign can send to a single endpoint in a 24-hour period.</p>
    #[serde(rename = "Daily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<i64>,
    /// <p>The length of time (in seconds) that the campaign can run before it ends and message deliveries stop. This duration begins at the scheduled start time for the campaign. The minimum value is 60.</p>
    #[serde(rename = "MaximumDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<i64>,
    /// <p>The number of messages that the campaign can send per second. The minimum value is 50, and the maximum is 20000.</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>The maximum number of messages that an individual campaign can send to a single endpoint over the course of the campaign.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// <p>Campaign definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignResponse {
    /// <p>Treatments that are defined in addition to the default treatment.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<TreatmentResource>>,
    /// <p>The ID of the application to which the campaign applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date the campaign was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The status of the campaign&#39;s default treatment. Only present for A/B test campaigns.</p>
    #[serde(rename = "DefaultState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_state: Option<CampaignState>,
    /// <p>A description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of end users who will not receive messages from this campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>Campaign hook information.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>The unique campaign ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The date the campaign was last updated in ISO 8601 format.  </p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The campaign limits settings.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The custom name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The ID of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
    /// <p>The campaign status.</p>
    ///
    /// <p>An A/B test campaign will have a status of COMPLETED only when all treatments have a status of COMPLETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    /// <p>The campaign version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>SMS message configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignSmsMessage {
    /// <p>The SMS text body.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Is this is a transactional SMS message, otherwise a promotional message.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>Sender ID of sent message.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// <p>State of the Campaign</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignState {
    /// <p>The status of the campaign, or the status of a treatment that belongs to an A/B test campaign.</p>
    ///
    /// <p>Valid values: SCHEDULED, EXECUTING, PENDING<em>NEXT</em>RUN, COMPLETED, PAUSED</p>
    #[serde(rename = "CampaignStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_status: Option<String>,
}

/// <p>List of available campaigns.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CampaignsResponse {
    /// <p>A list of campaigns.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<CampaignResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Base definition for channel response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Get channels definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelsResponse {
    /// <p>A map of channels, with the ChannelType as the key and the Channel as the value.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<::std::collections::HashMap<String, ChannelResponse>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAppRequest {
    #[serde(rename = "CreateApplicationRequest")]
    pub create_application_request: CreateApplicationRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

/// <p>Application Request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>The display name of the application. Used in the Amazon Pinpoint console.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateExportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ExportJobRequest")]
    pub export_job_request: ExportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateImportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ImportJobRequest")]
    pub import_job_request: ImportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

/// <p>The default message to use across all channels.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DefaultMessage {
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Default Push Notification Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DefaultPushNotificationMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>Indicates if the message should display on the recipient&#39;s device. You can use silent pushes for remote configuration or to deliver messages to in-app notification centers.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAdmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsVoipChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApnsVoipSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUserEndpointsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVoiceChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

/// <p>Message definitions for the default message and any messages that are tailored for specific channels.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DirectMessageConfiguration {
    /// <p>The message to ADM channels. Overrides the default push notification message.</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<ADMMessage>,
    /// <p>The message to APNS channels. Overrides the default push notification message.</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<APNSMessage>,
    /// <p>The message to Baidu GCM channels. Overrides the default push notification message.</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<BaiduMessage>,
    /// <p>The default message for all channels.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<DefaultMessage>,
    /// <p>The default push notification message for all push channels.</p>
    #[serde(rename = "DefaultPushNotificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_push_notification_message: Option<DefaultPushNotificationMessage>,
    /// <p>The message to Email channels. Overrides the default message.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<EmailMessage>,
    /// <p>The message to GCM channels. Overrides the default push notification message.</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<GCMMessage>,
    /// <p>The message to SMS channels. Overrides the default message.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<SMSMessage>,
    /// <p>The message to Voice channels. Overrides the default message.</p>
    #[serde(rename = "VoiceMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_message: Option<VoiceMessage>,
}

/// <p>Email Channel Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EmailChannelRequest {
    /// <p>The configuration set that you want to use when you send email using the Pinpoint Email API.</p>
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The email address used to send emails from.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The ARN of an identity verified with SES.</p>
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>The ARN of an IAM Role used to submit events to Mobile Analytics&#39; event ingestion service</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Email Channel Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EmailChannelResponse {
    /// <p>The unique ID of the application to which the email channel belongs.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The configuration set that you want to use when you send email using the Pinpoint Email API.</p>
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The email address used to send emails from.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The ARN of an identity verified with SES.</p>
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Messages per second that can be sent</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>Platform type. Will be &quot;EMAIL&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The ARN of an IAM Role used to submit events to Mobile Analytics&#39; event ingestion service</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Email Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EmailMessage {
    /// <p>The body of the email message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The email address that bounces and complaints will be forwarded to when feedback forwarding is enabled.</p>
    #[serde(rename = "FeedbackForwardingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_address: Option<String>,
    /// <p>The email address used to send the email from. Defaults to use FromAddress specified in the Email Channel.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>An email represented as a raw MIME message.</p>
    #[serde(rename = "RawEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_email: Option<RawEmail>,
    /// <p>The reply-to email address(es) for the email. If the recipient replies to the email, each reply-to address will receive the reply.</p>
    #[serde(rename = "ReplyToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>An email composed of a subject, a text part and a html part.</p>
    #[serde(rename = "SimpleEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_email: Option<SimpleEmail>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Endpoint update request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointBatchItem {
    /// <p>The destination for messages that you send to this endpoint. The address varies by channel. For mobile push channels, use the token provided by the push notification service, such as the APNs device token or the FCM registration token. For the SMS channel, use a phone number in E.164 format, such as +12065550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The last time the endpoint was updated. Provided in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique Id for the Endpoint in the batch.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint batch update request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointBatchRequest {
    /// <p>List of items to update. Maximum 100 items</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EndpointBatchItem>>,
}

/// <p>Demographic information about the endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDemographic {
    /// <p>The version of the application associated with the endpoint.</p>
    #[serde(rename = "AppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// <p>The endpoint locale in the following format: The ISO 639-1 alpha-2 code, followed by an underscore, followed by an ISO 3166-1 alpha-2 value.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The manufacturer of the endpoint device, such as Apple or Samsung.</p>
    #[serde(rename = "Make")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// <p>The model name or number of the endpoint device, such as iPhone.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The model version of the endpoint device.</p>
    #[serde(rename = "ModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>The platform of the endpoint device, such as iOS or Android.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The platform version of the endpoint device.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The timezone of the endpoint. Specified as a tz database value, such as Americas/Los_Angeles.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>A complex object that holds the status code and message as a result of processing an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointItemResponse {
    /// <p>A custom message associated with the registration of an endpoint when issuing a response.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code associated with the merging of an endpoint when issuing a response.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

/// <p>Location data for the endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointLocation {
    /// <p>The city where the endpoint is located.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The two-letter code for the country or region of the endpoint. Specified as an ISO 3166-1 alpha-2 code, such as &quot;US&quot; for the United States.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The latitude of the endpoint location, rounded to one decimal place.</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// <p>The longitude of the endpoint location, rounded to one decimal place.</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// <p>The postal code or zip code of the endpoint.</p>
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// <p>The region of the endpoint location. For example, in the United States, this corresponds to a state.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>The result from sending a message to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointMessageResult {
    /// <p>Address that endpoint message was delivered to.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The delivery status of the message. Possible values:</p>
    ///
    /// <p>SUCCESS - The message was successfully delivered to the endpoint.</p>
    ///
    /// <p>TRANSIENT_FAILURE - A temporary error occurred. Amazon Pinpoint will attempt to deliver the message again later.</p>
    ///
    /// <p>FAILURE_PERMANENT - An error occurred when delivering the message to the endpoint. Amazon Pinpoint won&#39;t attempt to send the message again.</p>
    ///
    /// <p>TIMEOUT - The message couldn&#39;t be sent within the timeout period.</p>
    ///
    /// <p>QUIET_TIME - The local time for the endpoint was within the QuietTime for the campaign or app.</p>
    ///
    /// <p>DAILY_CAP - The endpoint has received the maximum number of messages it can receive within a 24-hour period.</p>
    ///
    /// <p>HOLDOUT - The endpoint was in a hold out treatment for the campaign.</p>
    ///
    /// <p>THROTTLED - Amazon Pinpoint throttled sending to this endpoint.</p>
    ///
    /// <p>EXPIRED - The endpoint address is expired.</p>
    ///
    /// <p>CAMPAIGN_CAP - The endpoint received the maximum number of messages allowed by the campaign.</p>
    ///
    /// <p>SERVICE_FAILURE - A service-level failure prevented Amazon Pinpoint from delivering the message.</p>
    ///
    /// <p>UNKNOWN - An unknown error occurred.</p>
    #[serde(rename = "DeliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    /// <p>Unique message identifier associated with the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>Downstream service status code.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// <p>Status message for message delivery.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>If token was updated as part of delivery. (This is GCM Specific)</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>An endpoint update request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointRequest {
    /// <p>The destination for messages that you send to this endpoint. The address varies by channel. For mobile push channels, use the token provided by the push notification service, such as the APNs device token or the FCM registration token. For the SMS channel, use a phone number in E.164 format, such as +12065550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the values [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create a segment of users to engage with a messaging campaign.</p>
    ///
    /// <p>The following characters are not recommended in attribute names: # : ? \ /. The Amazon Pinpoint console does not display attributes that include these characters in the name. This limitation does not apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>Demographic attributes for the endpoint.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time when the endpoint was updated, shown in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint response</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointResponse {
    /// <p>The address of the endpoint as provided by your push provider. For example, the DeviceToken or RegistrationId.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The ID of the application that is associated with the endpoint.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>Custom attributes that describe the endpoint by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the following values: [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create segments.</p>
    ///
    /// <p>The Amazon Pinpoint console can&#39;t display attribute names that include the following characters: hash/pound sign (#), colon (:), question mark (?), backslash (), and forward slash (/). For this reason, you should avoid using these characters in the names of custom attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: GCM | APNS | APNS<em>SANDBOX | APNS</em>VOIP | APNS<em>VOIP</em>SANDBOX | ADM | SMS | EMAIL | BAIDU</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A number from 0-99 that represents the cohort the endpoint is assigned to. Endpoints are grouped into cohorts randomly, and each cohort contains approximately 1 percent of the endpoints for an app. Amazon Pinpoint assigns cohorts to the holdout or treatment allocations for a campaign.</p>
    #[serde(rename = "CohortId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_id: Option<String>,
    /// <p>The date and time when the endpoint was created, shown in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time when the endpoint was last updated, shown in ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Unused.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique ID that you assigned to the endpoint. The ID should be a globally unique identifier (GUID) to ensure that it doesn&#39;t conflict with other endpoint IDs associated with the application.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique ID for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Endpoint send configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EndpointSendConfiguration {
    /// <p>Body override. If specified will override default body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>A map of custom attributes to attributes to be attached to the message for this address. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of substitution values for the message to be merged with the DefaultMessage&#39;s substitutions. Substitutions on this map take precedence over the all other substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Title override. If specified will override default title if applicable.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Endpoint user specific custom userAttributes</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointUser {
    /// <p>Custom attributes that describe the user by associating a name with an array of values. For example, an attribute named &quot;interests&quot; might have the following values: [&quot;science&quot;, &quot;politics&quot;, &quot;travel&quot;]. You can use these attributes as selection criteria when you create segments.</p>
    ///
    /// <p>The Amazon Pinpoint console can&#39;t display attribute names that include the following characters: hash/pound sign (#), colon (:), question mark (?), backslash (), and forward slash (/). For this reason, you should avoid using these characters in the names of custom attributes.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>List of endpoints</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EndpointsResponse {
    /// <p>The list of endpoints.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EndpointResponse>>,
}

/// <p>Model for creating or updating events.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Event {
    /// <p>The package name associated with the app that&#39;s recording the event.</p>
    #[serde(rename = "AppPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_package_name: Option<String>,
    /// <p>The title of the app that&#39;s recording the event.</p>
    #[serde(rename = "AppTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    /// <p>The version number of the app that&#39;s recording the event.</p>
    #[serde(rename = "AppVersionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version_code: Option<String>,
    /// <p>Custom attributes that are associated with the event you&#39;re adding or updating.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the SDK that&#39;s running on the client device.</p>
    #[serde(rename = "ClientSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_sdk_version: Option<String>,
    /// <p>The name of the custom event that you&#39;re recording.</p>
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>Custom metrics related to the event.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>The name of the SDK that&#39;s being used to record the event.</p>
    #[serde(rename = "SdkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_name: Option<String>,
    /// <p>Information about the session in which the event occurred.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    /// <p>The date and time when the event occurred, in ISO 8601 format.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// <p>Event dimensions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDimensions {
    /// <p>Custom attributes that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create an event filter.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    /// <p>The name of the event that causes the campaign to be sent. This can be a standard event type that Amazon Pinpoint generates, such as _session.start, or a custom event that&#39;s specific to your app.</p>
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<SetDimension>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create an event filter.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricDimension>>,
}

/// <p>A complex object that holds the status code and message as a result of processing an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventItemResponse {
    /// <p>A custom message that is associated with the processing of an event.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status returned in the response as a result of processing the event.</p>
    ///
    /// <p>Possible values: 400 (for invalid events) and 202 (for events that were accepted).</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

/// <p>Model for an event publishing subscription export.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventStream {
    /// <p>The ID of the application from which events should be published.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
    /// Firehose ARN: arn:aws:firehose:REGION:ACCOUNT<em>ID:deliverystream/STREAM</em>NAME
    /// Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT<em>ID:stream/STREAM</em>NAME</p>
    #[serde(rename = "DestinationStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_stream_arn: Option<String>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The date the event stream was last updated in ISO 8601 format.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The IAM user who last modified the event stream.</p>
    #[serde(rename = "LastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>A batch of PublicEndpoints and Events to process.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EventsBatch {
    /// <p>The PublicEndpoint attached to the EndpointId from the request.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<PublicEndpoint>,
    /// <p>An object that contains a set of events associated with the endpoint.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<::std::collections::HashMap<String, Event>>,
}

/// <p>A set of events to process.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EventsRequest {
    /// <p>A batch of events to process. Each BatchItem consists of an endpoint ID as the key, and an EventsBatch object as the value.</p>
    #[serde(rename = "BatchItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_item: Option<::std::collections::HashMap<String, EventsBatch>>,
}

/// <p>Custom messages associated with events.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventsResponse {
    /// <p>A map that contains a multipart response for each endpoint. Each item in this object uses the endpoint ID as the key, and the item response as the value.</p>
    ///
    /// <p>If no item response exists, the value can also be one of the following: 202 (if the request was processed successfully) or 400 (if the payload was invalid, or required fields were missing).</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<::std::collections::HashMap<String, ItemResponse>>,
}

/// <p>Export job request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that endpoints will be exported to.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that will receive the export. The location is typically a folder with multiple files.</p>
    ///
    /// <p>The URL should follow this format: s3://bucket-name/folder-name/</p>
    ///
    /// <p>Amazon Pinpoint will export endpoints to this location.</p>
    #[serde(rename = "S3UrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_prefix: Option<String>,
    /// <p>The ID of the segment to export endpoints from. If not present, Amazon Pinpoint exports all of the endpoints that belong to the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to export if specified.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Export job resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobResource {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that endpoints will be exported to.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A URL that points to the location within an Amazon S3 bucket that will receive the export. The location is typically a folder with multiple files.</p>
    ///
    /// <p>The URL should follow this format: s3://bucket-name/folder-name/</p>
    ///
    /// <p>Amazon Pinpoint will export endpoints to this location.</p>
    #[serde(rename = "S3UrlPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_prefix: Option<String>,
    /// <p>The ID of the segment to export endpoints from. If not present, Amazon Pinpoint exports all of the endpoints that belong to the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to export if specified.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Export job response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobResponse {
    /// <p>The unique ID of the application associated with the export job.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The number of pieces that have successfully completed as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date the job completed in ISO 8601 format.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date the job was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The export job settings.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ExportJobResource>,
    /// <p>The number of pieces that failed to be processed as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>Provides up to 100 of the first failed entries for the job, if any exist.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the job.
    /// Valid values: CREATED, INITIALIZING, PROCESSING, COMPLETING, COMPLETED, FAILING, FAILED</p>
    ///
    /// <p>The job status is FAILED if one or more pieces failed.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The number of endpoints that were not processed; for example, because of syntax errors.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be processed to finish the job. Each piece is an approximately equal portion of the endpoints.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The number of endpoints that were processed by the job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. Will be &#39;EXPORT&#39;.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Export job list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportJobsResponse {
    /// <p>A list of export jobs for the application.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ExportJobResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Google Cloud Messaging credentials</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GCMChannelRequest {
    /// <p>Platform credential API key from Google.</p>
    #[serde(rename = "ApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Google Cloud Messaging channel definition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GCMChannelResponse {
    /// <p>The ID of the application to which the channel applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>When was this segment created</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The GCM API key from Google.</p>
    #[serde(rename = "Credential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used. Present only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The platform type. Will be GCM</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>GCM Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GCMMessage {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign: OPEN<em>APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action. DEEP</em>LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app. URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify. Possible values include: OPEN<em>APP | DEEP</em>LINK | URL</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body of the notification.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>This parameter identifies a group of messages (e.g., with collapse_key: &quot;Updates Available&quot;) that can be collapsed, so that only the last message gets sent when delivery can be resumed. This is intended to avoid sending too many of the same messages when the device comes back online or becomes active.</p>
    #[serde(rename = "CollapseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
    /// <p>The data payload used for a silent push. This payload is added to the notifications&#39; data.pinpoint.jsonBody&#39; object</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your application.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL that points to an image used as the large icon to the notification content view.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The message priority. Amazon Pinpoint uses this value to set the FCM or GCM priority parameter when it sends the message. Accepts the following values:</p>
    ///
    /// <p>&quot;Normal&quot; - Messages might be delayed. Delivery is optimized for battery usage on the receiving device. Use normal priority unless immediate delivery is required.</p>
    ///
    /// <p>&quot;High&quot; - Messages are sent immediately and might wake a sleeping device.</p>
    ///
    /// <p>The equivalent values for APNs messages are &quot;5&quot; and &quot;10&quot;. Amazon Pinpoint accepts these values here and converts them.</p>
    ///
    /// <p>For more information, see About FCM Messages in the Firebase documentation.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>This parameter specifies the package name of the application where the registration tokens must match in order to receive the message.</p>
    #[serde(rename = "RestrictedPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,
    /// <p>Indicates if the message should display on the users device. Silent pushes can be used for Remote Configuration and Phone Home use cases.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL that points to an image used as the small icon for the notification which will be used to represent the notification in the status bar and content view</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>Indicates a sound to play when the device receives the notification. Supports default, or the filename of a sound resource bundled in the app. Android sound files must reside in /res/raw/</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The length of time (in seconds) that FCM or GCM stores and attempts to deliver the message. If unspecified, the value defaults to the maximum, which is 2,419,200 seconds (28 days). Amazon Pinpoint uses this value to set the FCM or GCM time<em>to</em>live parameter.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>GPS coordinates</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSCoordinates {
    /// <p>Latitude</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// <p>Longitude</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// <p>GPS point location dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSPointDimension {
    /// <p>Coordinate to measure distance from.</p>
    #[serde(rename = "Coordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GPSCoordinates>,
    /// <p>Range in kilometers from the coordinate.</p>
    #[serde(rename = "RangeInKilometers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_kilometers: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAdmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsVoipChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApnsVoipSandboxChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApplicationSettingsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppsRequest {
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppsResponse {
    #[serde(rename = "ApplicationsResponse")]
    pub applications_response: ApplicationsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignActivitiesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignActivitiesResponse {
    #[serde(rename = "ActivitiesResponse")]
    pub activities_response: ActivitiesResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignVersionRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The version of the campaign.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignVersionResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignVersionsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignVersionsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCampaignsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCampaignsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetChannelsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetChannelsResponse {
    #[serde(rename = "ChannelsResponse")]
    pub channels_response: ChannelsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetImportJobRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetImportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentExportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentImportJobsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentVersionRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The segment version.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentVersionResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentVersionsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentVersionsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSegmentsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of entries you want on each page in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSegmentsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUserEndpointsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVoiceChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

/// <p>Import job request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportJobRequest {
    /// <p>Sets whether the endpoints create a segment when they are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sets whether the endpoints are registered with Amazon Pinpoint when they are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The URL of the S3 bucket that contains the segment information to import. The location can be a folder or a single file. The URL should use the following format: s3://bucket-name/folder-name/file-name</p>
    ///
    /// <p>Amazon Pinpoint imports endpoints from this location and any subfolders it contains.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The ID of the segment to update if the import job is meant to update an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>A custom name for the segment created by the import job. Use if DefineSegment is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Import job resource</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobResource {
    /// <p>Sets whether the endpoints create a segment when they are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sets whether the endpoints are registered with Amazon Pinpoint when they are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the Amazon S3 location that contains the endpoints to import.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The URL of the S3 bucket that contains the segment information to import. The location can be a folder or a single file. The URL should use the following format: s3://bucket-name/folder-name/file-name</p>
    ///
    /// <p>Amazon Pinpoint imports endpoints from this location and any subfolders it contains.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The ID of the segment to update if the import job is meant to update an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>A custom name for the segment created by the import job. Use if DefineSegment is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Import job response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobResponse {
    /// <p>The unique ID of the application to which the import job applies.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The number of pieces that have successfully imported as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date the import job completed in ISO 8601 format.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date the import job was created in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The import job settings.</p>
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ImportJobResource>,
    /// <p>The number of pieces that have failed to import as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>Provides up to 100 of the first failed entries for the job, if any exist.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique ID of the import job.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the import job.
    /// Valid values: CREATED, INITIALIZING, PROCESSING, COMPLETING, COMPLETED, FAILING, FAILED</p>
    ///
    /// <p>The job status is FAILED if one or more pieces failed to import.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The number of endpoints that failed to import; for example, because of syntax errors.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be imported to finish the job. Each piece is an approximately equal portion of the endpoints to import.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The number of endpoints that were processed by the import job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. Will be Import.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Import job list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportJobsResponse {
    /// <p>A list of import jobs for the application.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ImportJobResponse>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The response that&#39;s provided after registering the endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ItemResponse {
    /// <p>The response received after the endpoint was accepted.</p>
    #[serde(rename = "EndpointItemResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_item_response: Option<EndpointItemResponse>,
    /// <p>A multipart response object that contains a key and value for each event ID in the request. In each object, the event ID is the key, and an EventItemResponse object is the value.</p>
    #[serde(rename = "EventsItemResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_item_response: Option<::std::collections::HashMap<String, EventItemResponse>>,
}

/// <p>Message to send</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// <p>The action that occurs if the user taps a push notification delivered by the campaign:
    /// OPEN_APP - Your app launches, or it becomes the foreground app if it has been sent to the background. This is the default action.</p>
    ///
    /// <p>DEEP_LINK - Uses deep linking features in iOS and Android to open your app and display a designated user interface within the app.</p>
    ///
    /// <p>URL - The default mobile browser on the user&#39;s device launches and opens a web page at the URL you specify.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body. Can include up to 140 characters.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The URL that points to the icon image for the push notification icon, for example, the app icon.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL that points to the small icon image for the push notification icon, for example, the app icon.</p>
    #[serde(rename = "ImageSmallIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_small_icon_url: Option<String>,
    /// <p>The URL that points to an image used in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The JSON payload used for a silent push.</p>
    #[serde(rename = "JsonBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<String>,
    /// <p>A URL that refers to the location of an image or video that you want to display in the push notification.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The Raw JSON formatted string to be used as the payload. This value overrides the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Indicates if the message should display on the users device.</p>
    ///
    /// <p>Silent pushes can be used for Remote Configuration and Phone Home use cases. </p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>This parameter specifies how long (in seconds) the message should be kept if the service is unable to deliver the notification the first time. If the value is 0, it treats the notification as if it expires immediately and does not store the notification or attempt to redeliver it. This value is converted to the expiration field when sent to the service. It only applies to APNs and GCM</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The message title that displays above the message on the user&#39;s device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the user&#39;s mobile browser. Used if the value for Action is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Simple message object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageBody {
    /// <p>The error message that&#39;s returned from the API.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The unique message body ID.</p>
    #[serde(rename = "RequestID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Message configuration for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageConfiguration {
    /// <p>The message that the campaign delivers to ADM channels. Overrides the default message.</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<Message>,
    /// <p>The message that the campaign delivers to APNS channels. Overrides the default message.</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<Message>,
    /// <p>The message that the campaign delivers to Baidu channels. Overrides the default message.</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<Message>,
    /// <p>The default message for all channels.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<Message>,
    /// <p>The email message configuration.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<CampaignEmailMessage>,
    /// <p>The message that the campaign delivers to GCM channels. Overrides the default message.</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<Message>,
    /// <p>The SMS message configuration.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<CampaignSmsMessage>,
}

/// <p>Send message request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MessageRequest {
    /// <p>A map of key-value pairs, where each key is an address and each value is an AddressConfiguration object. An address can be a push notification token, a phone number, or an email address.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<::std::collections::HashMap<String, AddressConfiguration>>,
    /// <p>A map of custom attributes to attributes to be attached to the message. This payload is added to the push notification&#39;s &#39;data.pinpoint&#39; object or added to the email/sms delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map of key-value pairs, where each key is an endpoint ID and each value is an EndpointSendConfiguration object. Within an EndpointSendConfiguration object, you can tailor the message for an endpoint by specifying message overrides or substitutions.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, EndpointSendConfiguration>>,
    /// <p>Message configuration.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<DirectMessageConfiguration>,
    /// <p>A unique ID that you can use to trace a message. This ID is visible to recipients.</p>
    #[serde(rename = "TraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

/// <p>Send message response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageResponse {
    /// <p>Application id of the message.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>A map containing a multi part response for each address, with the endpointId as the key and the result as the value.</p>
    #[serde(rename = "EndpointResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_result: Option<::std::collections::HashMap<String, EndpointMessageResult>>,
    /// <p>Original request Id for which this message was delivered.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>A map containing a multi part response for each address, with the address as the key(Email address, phone number or push token) and the result as the value.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<::std::collections::HashMap<String, MessageResult>>,
}

/// <p>The result from sending a message to an address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MessageResult {
    /// <p>The delivery status of the message. Possible values:</p>
    ///
    /// <p>SUCCESS - The message was successfully delivered to the endpoint.</p>
    ///
    /// <p>TRANSIENT_FAILURE - A temporary error occurred. Amazon Pinpoint will attempt to deliver the message again later.</p>
    ///
    /// <p>FAILURE_PERMANENT - An error occurred when delivering the message to the endpoint. Amazon Pinpoint won&#39;t attempt to send the message again.</p>
    ///
    /// <p>TIMEOUT - The message couldn&#39;t be sent within the timeout period.</p>
    ///
    /// <p>QUIET_TIME - The local time for the endpoint was within the QuietTime for the campaign or app.</p>
    ///
    /// <p>DAILY_CAP - The endpoint has received the maximum number of messages it can receive within a 24-hour period.</p>
    ///
    /// <p>HOLDOUT - The endpoint was in a hold out treatment for the campaign.</p>
    ///
    /// <p>THROTTLED - Amazon Pinpoint throttled sending to this endpoint.</p>
    ///
    /// <p>EXPIRED - The endpoint address is expired.</p>
    ///
    /// <p>CAMPAIGN_CAP - The endpoint received the maximum number of messages allowed by the campaign.</p>
    ///
    /// <p>SERVICE_FAILURE - A service-level failure prevented Amazon Pinpoint from delivering the message.</p>
    ///
    /// <p>UNKNOWN - An unknown error occurred.</p>
    #[serde(rename = "DeliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    /// <p>Unique message identifier associated with the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>Downstream service status code.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// <p>Status message for message delivery.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>If token was updated as part of delivery. (This is GCM Specific)</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>Custom metric dimension</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The operator that you&#39;re using to compare metric values. Possible values: GREATER<em>THAN, LESS</em>THAN, GREATER<em>THAN</em>OR<em>EQUAL, LESS</em>THAN<em>OR</em>EQUAL, or EQUAL</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>The value to be compared.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Phone Number Validate request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NumberValidateRequest {
    /// <p>(Optional) The two-character ISO country code for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "IsoCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    /// <p>The phone number to get information about. The phone number that you provide should include a country code. If the number doesn&#39;t include a valid country code, the operation might result in an error.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// <p>Phone Number Validate response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NumberValidateResponse {
    /// <p>The carrier or servive provider that the phone number is currently registered with.</p>
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// <p>The city where the phone number was originally registered.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The cleansed phone number, shown in E.164 format.</p>
    #[serde(rename = "CleansedPhoneNumberE164")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_e164: Option<String>,
    /// <p>The cleansed phone number, shown in the local phone number format.</p>
    #[serde(rename = "CleansedPhoneNumberNational")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_national: Option<String>,
    /// <p>The country or region where the phone number was originally registered.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The two-character ISO code for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_iso_2: Option<String>,
    /// <p>The numeric code for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_numeric: Option<String>,
    /// <p>The county where the phone number was originally registered.</p>
    #[serde(rename = "County")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// <p>The two-character code (in ISO 3166-1 alpha-2 format) for the country or region in the request body.</p>
    #[serde(rename = "OriginalCountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_country_code_iso_2: Option<String>,
    /// <p>The phone number that you included in the request body.</p>
    #[serde(rename = "OriginalPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_phone_number: Option<String>,
    /// <p>A description of the phone type. Possible values are MOBILE, LANDLINE, VOIP, INVALID, PREPAID, and OTHER.</p>
    #[serde(rename = "PhoneType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    /// <p>The phone type, represented by an integer. Possible values include 0 (MOBILE), 1 (LANDLINE), 2 (VOIP), 3 (INVALID), 4 (OTHER), and 5 (PREPAID).</p>
    #[serde(rename = "PhoneTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type_code: Option<i64>,
    /// <p>The time zone for the location where the phone number was originally registered.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The postal code for the location where the phone number was originally registered.</p>
    #[serde(rename = "ZipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PhoneNumberValidateRequest {
    #[serde(rename = "NumberValidateRequest")]
    pub number_validate_request: NumberValidateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PhoneNumberValidateResponse {
    #[serde(rename = "NumberValidateResponse")]
    pub number_validate_response: NumberValidateResponse,
}

/// <p>Public endpoint attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublicEndpoint {
    /// <p>The unique identifier for the recipient. For example, an address could be a device token, email address, or mobile phone number.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Custom attributes that your app reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create a segment.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel type.</p>
    ///
    /// <p>Valid values: APNS, GCM</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The endpoint demographic attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time when the endpoint was last updated, in  ISO 8601 format.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>The status of the endpoint. If the update fails, the value is INACTIVE. If the endpoint is updated successfully, the value is ACTIVE.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The endpoint location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>Custom metrics that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Indicates whether a user has opted out of receiving messages with one of the following values:</p>
    ///
    /// <p>ALL - User has opted out of all messages.</p>
    ///
    /// <p>NONE - Users has not opted out and receives all messages.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>A unique identifier that is generated each time the endpoint is updated.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Custom user-specific attributes that your app reports to Amazon Pinpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventStreamRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteEventStream")]
    pub write_event_stream: WriteEventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEventsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EventsRequest")]
    pub events_request: EventsRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEventsResponse {
    #[serde(rename = "EventsResponse")]
    pub events_response: EventsResponse,
}

/// <p>Quiet Time</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuietTime {
    /// <p>The time at which quiet time should end. The value that you specify has to be in HH:mm format, where HH is the hour in 24-hour format (with a leading zero, if applicable), and mm is the minutes. For example, use 02:30 to represent 2:30 AM, or 14:30 to represent 2:30 PM.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The time at which quiet time should begin. The value that you specify has to be in HH:mm format, where HH is the hour in 24-hour format (with a leading zero, if applicable), and mm is the minutes. For example, use 02:30 to represent 2:30 AM, or 14:30 to represent 2:30 PM.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// <p>An email represented as a raw MIME message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RawEmail {
    /// <p>The raw email message itself. Then entire message must be base64-encoded.</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bytes::Bytes>,
}

/// <p>Define how a segment based on recency of use.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecencyDimension {
    /// <p>The length of time during which users have been active or inactive with your app.</p>
    ///
    /// <p>Valid values: HR<em>24, DAY</em>7, DAY<em>14, DAY</em>30</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// <p>The recency dimension type:</p>
    ///
    /// <p>ACTIVE - Users who have used your app within the specified duration are included in the segment.</p>
    ///
    /// <p>INACTIVE - Users who have not used your app within the specified duration are included in the segment.</p>
    #[serde(rename = "RecencyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAttributesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>Type of attribute. Can be endpoint-custom-attributes, endpoint-custom-metrics, endpoint-user-attributes.</p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    #[serde(rename = "UpdateAttributesRequest")]
    pub update_attributes_request: UpdateAttributesRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveAttributesResponse {
    #[serde(rename = "AttributesResource")]
    pub attributes_resource: AttributesResource,
}

/// <p>SMS Channel Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SMSChannelRequest {
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Sender identifier of your messages.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>ShortCode registered with phone provider.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
}

/// <p>SMS Channel Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SMSChannelResponse {
    /// <p>The unique ID of the application to which the SMS channel belongs.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Not used. Retained for backwards compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who last updated this entry</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Platform type. Will be &quot;SMS&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Promotional messages per second that can be sent</p>
    #[serde(rename = "PromotionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_messages_per_second: Option<i64>,
    /// <p>Sender identifier of your messages.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>The short code registered with the phone provider.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    /// <p>Transactional messages per second that can be sent</p>
    #[serde(rename = "TransactionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_messages_per_second: Option<i64>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>SMS Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SMSMessage {
    /// <p>The body of the SMS message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The SMS program name that you provided to AWS Support when you requested your dedicated number.</p>
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// <p>Is this a transaction priority message or lower priority.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The phone number that the SMS message originates from. Specify one of the dedicated long codes or short codes that you requested from AWS Support and that is assigned to your account. If this attribute is not specified, Amazon Pinpoint randomly assigns a long code.</p>
    #[serde(rename = "OriginationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// <p>The sender ID that is shown as the message sender on the recipient&#39;s device. Support for sender IDs varies by country or region.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Shcedule that defines when a campaign is run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// <p>The scheduled time that the campaign ends in ISO 8601 format.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>Defines the type of events that can trigger the campaign. Used when the Frequency is set to EVENT.</p>
    #[serde(rename = "EventFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<CampaignEventFilter>,
    /// <p>How often the campaign delivers messages.</p>
    ///
    /// <p>Valid values:</p>
    ///
    /// <p>ONCE</p>
    ///
    /// <p>HOURLY</p>
    ///
    /// <p>DAILY</p>
    ///
    /// <p>WEEKLY</p>
    ///
    /// <p>MONTHLY</p>
    ///
    /// <p>EVENT</p>
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// <p>Indicates whether the campaign schedule takes effect according to each user&#39;s local time.</p>
    #[serde(rename = "IsLocalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_time: Option<bool>,
    /// <p>The default quiet time for the campaign. The campaign doesn&#39;t send messages to endpoints during the quiet time.</p>
    ///
    /// <p>Note: Make sure that your endpoints include the Demographics.Timezone attribute if you plan to enable a quiet time for your campaign. If your endpoints don&#39;t include this attribute, they&#39;ll receive the messages that you send them, even if quiet time is enabled.</p>
    ///
    /// <p>When you set up a campaign to use quiet time, the campaign doesn&#39;t send messages during the time range you specified, as long as all of the following are true:
    /// - The endpoint includes a valid Demographic.Timezone attribute.
    /// - The current time in the endpoint&#39;s time zone is later than or equal to the time specified in the QuietTime.Start attribute for the campaign.
    /// - The current time in the endpoint&#39;s time zone is earlier than or equal to the time specified in the QuietTime.End attribute for the campaign.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    /// <p>The scheduled time that the campaign begins in ISO 8601 format.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The starting UTC offset for the schedule if the value for isLocalTime is true</p>
    ///
    /// <p>Valid values:
    /// UTC
    /// UTC+01
    /// UTC+02
    /// UTC+03
    /// UTC+03:30
    /// UTC+04
    /// UTC+04:30
    /// UTC+05
    /// UTC+05:30
    /// UTC+05:45
    /// UTC+06
    /// UTC+06:30
    /// UTC+07
    /// UTC+08
    /// UTC+09
    /// UTC+09:30
    /// UTC+10
    /// UTC+10:30
    /// UTC+11
    /// UTC+12
    /// UTC+13
    /// UTC-02
    /// UTC-03
    /// UTC-04
    /// UTC-05
    /// UTC-06
    /// UTC-07
    /// UTC-08
    /// UTC-09
    /// UTC-10
    /// UTC-11</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Segment behavior dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentBehaviors {
    /// <p>The recency of use.</p>
    #[serde(rename = "Recency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency: Option<RecencyDimension>,
}

/// <p>Segment demographic dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentDemographics {
    /// <p>The app version criteria for the segment.</p>
    #[serde(rename = "AppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<SetDimension>,
    /// <p>The channel criteria for the segment.</p>
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<SetDimension>,
    /// <p>The device type criteria for the segment.</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<SetDimension>,
    /// <p>The device make criteria for the segment.</p>
    #[serde(rename = "Make")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<SetDimension>,
    /// <p>The device model criteria for the segment.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SetDimension>,
    /// <p>The device platform criteria for the segment.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SetDimension>,
}

/// <p>Segment dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentDimensions {
    /// <p>Custom segment attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    /// <p>The segment behaviors attributes.</p>
    #[serde(rename = "Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<SegmentBehaviors>,
    /// <p>The segment demographics attributes.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<SegmentDemographics>,
    /// <p>The segment location attributes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SegmentLocation>,
    /// <p>Custom segment metrics.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricDimension>>,
    /// <p>Custom segment user attributes.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
}

/// <p>Segment group definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroup {
    /// <p>List of dimensions to include or exclude.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<SegmentDimensions>>,
    /// <p>The base segment that you build your segment on. The source segment defines the starting &quot;universe&quot; of endpoints. When you add dimensions to the segment, it filters the source segment based on the dimensions that you specify. You can specify more than one dimensional segment. You can only specify one imported segment.</p>
    ///
    /// <p>NOTE: If you specify an imported segment for this attribute, the segment size estimate that appears in the Amazon Pinpoint console shows the size of the imported segment, without any filters applied to it.</p>
    #[serde(rename = "SourceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_segments: Option<Vec<SegmentReference>>,
    /// <p>Specify how to handle multiple source segments. For example, if you specify three source segments, should the resulting segment be based on any or all of the segments? Acceptable values: ANY or ALL.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>Specify how to handle multiple segment dimensions. For example, if you specify three dimensions, should the resulting segment include endpoints that are matched by all, any, or none of the dimensions? Acceptable values: ALL, ANY, or NONE.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Segment group definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroupList {
    /// <p>A set of segment criteria to evaluate.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
    /// <p>Specify how to handle multiple segment groups. For example, if the segment includes three segment groups, should the resulting segment include endpoints that are matched by all, any, or none of the segment groups you created. Acceptable values: ALL, ANY, or NONE.</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

/// <p>Segment import definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentImportResource {
    /// <p>The number of channel types in the imported segment.</p>
    #[serde(rename = "ChannelCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_counts: Option<::std::collections::HashMap<String, i64>>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to the ExternalID key in an IAM trust policy. Used by Amazon Pinpoint to assume an IAM role. This requirement is removed, and external IDs are not recommended for IAM roles assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the endpoint files that were imported to create this segment.
    /// Valid values: CSV, JSON</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that grants Amazon Pinpoint access to the endpoints in Amazon S3.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The URL of the S3 bucket that the segment was imported from.</p>
    #[serde(rename = "S3Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    /// <p>The number of endpoints that were successfully imported to create this segment.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>Segment location dimensions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentLocation {
    /// <p>The country or region, in ISO 3166-1 alpha-2 format.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<SetDimension>,
    /// <p>The GPS Point dimension.</p>
    #[serde(rename = "GPSPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_point: Option<GPSPointDimension>,
}

/// <p>Segment reference.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentReference {
    /// <p>A unique identifier for the segment.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>If specified contains a specific version of the segment included.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Segment definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentResponse {
    /// <p>The ID of the application that the segment applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the segment was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The segment dimensions attributes.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The unique segment ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The import job settings.</p>
    #[serde(rename = "ImportDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_definition: Option<SegmentImportResource>,
    /// <p>The date and time when the segment was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The name of the segment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A segment group, which consists of zero or more source segments, plus dimensions that are applied to those source segments.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    /// <p>The segment type:
    /// DIMENSIONAL - A dynamic segment built from selection criteria based on endpoint data reported by your app. You create this type of segment by using the segment builder in the Amazon Pinpoint console or by making a POST request to the segments resource.
    /// IMPORT - A static segment built from an imported set of endpoint definitions. You create this type of segment by importing a segment in the Amazon Pinpoint console or by making a POST request to the jobs/import resource.</p>
    #[serde(rename = "SegmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    /// <p>The segment version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Segments in your account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SegmentsResponse {
    /// <p>The list of segments.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<SegmentResponse>>,
    /// <p>An identifier used to retrieve the next page of results. The token is null if no additional pages exist.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendMessagesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "MessageRequest")]
    pub message_request: MessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendMessagesResponse {
    #[serde(rename = "MessageResponse")]
    pub message_response: MessageResponse,
}

/// <p>Send message request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendUsersMessageRequest {
    /// <p>A map of custom attribute-value pairs. Amazon Pinpoint adds these attributes to the data.pinpoint object in the body of the push notification payload. Amazon Pinpoint also provides these attributes in the events that it generates for users-messages deliveries.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>Message definitions for the default message and any messages that are tailored for specific channels.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<DirectMessageConfiguration>,
    /// <p>A unique ID that you can use to trace a message. This ID is visible to recipients.</p>
    #[serde(rename = "TraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// <p>A map that associates user IDs with EndpointSendConfiguration objects. Within an EndpointSendConfiguration object, you can tailor the message for a user by specifying message overrides or substitutions.</p>
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<::std::collections::HashMap<String, EndpointSendConfiguration>>,
}

/// <p>User send message response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendUsersMessageResponse {
    /// <p>The unique ID of the Amazon Pinpoint project used to send the message.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The unique ID assigned to the users-messages request.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>An object that shows the endpoints that were messaged for each user. The object provides a list of user IDs. For each user ID, it provides the endpoint IDs that were messaged. For each endpoint ID, it provides an EndpointMessageResult object.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<
        ::std::collections::HashMap<
            String,
            ::std::collections::HashMap<String, EndpointMessageResult>,
        >,
    >,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendUsersMessagesRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SendUsersMessageRequest")]
    pub send_users_message_request: SendUsersMessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendUsersMessagesResponse {
    #[serde(rename = "SendUsersMessageResponse")]
    pub send_users_message_response: SendUsersMessageResponse,
}

/// <p>Information about a session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Session {
    /// <p>The duration of the session, in milliseconds.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>A unique identifier for the session.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time when the session began.</p>
    #[serde(rename = "StartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<String>,
    /// <p>The date and time when the session ended.</p>
    #[serde(rename = "StopTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timestamp: Option<String>,
}

/// <p>Dimension specification of a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDimension {
    /// <p>The type of dimension:</p>
    ///
    /// <p>INCLUSIVE - Endpoints that match the criteria are included in the segment.</p>
    ///
    /// <p>EXCLUSIVE - Endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "DimensionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    /// <p>The criteria values for the segment dimension. Endpoints with matching attribute values are included or excluded from the segment, depending on the setting for Type.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>An email composed of a subject, a text part and a html part.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SimpleEmail {
    /// <p>The content of the message, in HTML format. Use this for email clients that can process HTML. You can include clickable links, formatted text, and much more in an HTML message.</p>
    #[serde(rename = "HtmlPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<SimpleEmailPart>,
    /// <p>The subject of the message: A short summary of the content, which will appear in the recipient&#39;s inbox.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SimpleEmailPart>,
    /// <p>The content of the message, in text format. Use this for text-based email clients, or clients on high-latency networks (such as mobile devices).</p>
    #[serde(rename = "TextPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<SimpleEmailPart>,
}

/// <p>Textual email data, plus an optional character set specification.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SimpleEmailPart {
    /// <p>The character set of the content.</p>
    #[serde(rename = "Charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// <p>The textual data of the content.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// <p>Treatment resource</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TreatmentResource {
    /// <p>The unique treatment ID.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users for this treatment.</p>
    #[serde(rename = "SizePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<i64>,
    /// <p>The treatment status.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAdmChannelRequest {
    #[serde(rename = "ADMChannelRequest")]
    pub adm_channel_request: ADMChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsChannelRequest {
    #[serde(rename = "APNSChannelRequest")]
    pub apns_channel_request: APNSChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsSandboxChannelRequest {
    #[serde(rename = "APNSSandboxChannelRequest")]
    pub apns_sandbox_channel_request: APNSSandboxChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsVoipChannelRequest {
    #[serde(rename = "APNSVoipChannelRequest")]
    pub apns_voip_channel_request: APNSVoipChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApnsVoipSandboxChannelRequest {
    #[serde(rename = "APNSVoipSandboxChannelRequest")]
    pub apns_voip_sandbox_channel_request: APNSVoipSandboxChannelRequest,
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationSettingsRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteApplicationSettingsRequest")]
    pub write_application_settings_request: WriteApplicationSettingsRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

/// <p>Update attributes request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAttributesRequest {
    /// <p>The GLOB wildcard for removing the attributes in the application</p>
    #[serde(rename = "Blacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBaiduChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "BaiduChannelRequest")]
    pub baidu_channel_request: BaiduChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCampaignRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEmailChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EmailChannelRequest")]
    pub email_channel_request: EmailChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
    #[serde(rename = "EndpointRequest")]
    pub endpoint_request: EndpointRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEndpointResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEndpointsBatchRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EndpointBatchRequest")]
    pub endpoint_batch_request: EndpointBatchRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateEndpointsBatchResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGcmChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "GCMChannelRequest")]
    pub gcm_channel_request: GCMChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSegmentRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique ID of the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSmsChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SMSChannelRequest")]
    pub sms_channel_request: SMSChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVoiceChannelRequest {
    /// <p>The unique ID of your Amazon Pinpoint application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "VoiceChannelRequest")]
    pub voice_channel_request: VoiceChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

/// <p>Voice Channel Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VoiceChannelRequest {
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Voice Channel Response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VoiceChannelResponse {
    /// <p>Application id</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date that the settings were last updated in ISO 8601 format.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>If the channel is enabled for sending messages.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Channel ID. Not used, only for backwards compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Is this channel archived</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>Who made the last change</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Last date this was updated</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>Platform type. Will be &quot;Voice&quot;</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Version of channel</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Voice Message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VoiceMessage {
    /// <p>The message body of the notification, the email body or the text message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>Language of sent message</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Is the number from the pool or messaging service to send from.</p>
    #[serde(rename = "OriginationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// <p>Default message substitutions. Can be overridden by individual address substitutions.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>Voice ID of sent message.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

/// <p>Creating application setting request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteApplicationSettingsRequest {
    /// <p>Default campaign hook information.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>The CloudWatchMetrics settings for the app.</p>
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    /// <p>The limits that apply to each campaign in the project by default. Campaigns can also have their own limits, which override the settings at the project level.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for the app. Campaigns in the app don&#39;t send messages to endpoints during the quiet time.</p>
    ///
    /// <p>Note: Make sure that your endpoints include the Demographics.Timezone attribute if you plan to enable a quiet time for your app. If your endpoints don&#39;t include this attribute, they&#39;ll receive the messages that you send them, even if quiet time is enabled.</p>
    ///
    /// <p>When you set up an app to use quiet time, campaigns in that app don&#39;t send messages during the time range you specified, as long as all of the following are true:
    /// - The endpoint includes a valid Demographic.Timezone attribute.
    /// - The current time in the endpoint&#39;s time zone is later than or equal to the time specified in the QuietTime.Start attribute for the app (or campaign, if applicable).
    /// - The current time in the endpoint&#39;s time zone is earlier than or equal to the time specified in the QuietTime.End attribute for the app (or campaign, if applicable).</p>
    ///
    /// <p>Individual campaigns within the app can have their own quiet time settings, which override the quiet time settings at the app level.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Used to create a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteCampaignRequest {
    /// <p>Treatments that are defined in addition to the default treatment.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<WriteTreatmentResource>>,
    /// <p>A description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of end users who will not receive messages from this campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>Campaign hook information.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>Indicates whether the campaign is paused. A paused campaign does not send messages unless you resume it by setting IsPaused to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The campaign limits settings.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The custom name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The ID of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to which the campaign sends messages.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

/// <p>Request to save an EventStream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteEventStream {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis stream or Firehose delivery stream to which you want to publish events.
    /// Firehose ARN: arn:aws:firehose:REGION:ACCOUNT<em>ID:deliverystream/STREAM</em>NAME
    /// Kinesis ARN: arn:aws:kinesis:REGION:ACCOUNT<em>ID:stream/STREAM</em>NAME</p>
    #[serde(rename = "DestinationStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_stream_arn: Option<String>,
    /// <p>The IAM role that authorizes Amazon Pinpoint to publish events to the stream in your account.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Segment definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteSegmentRequest {
    /// <p>The segment dimensions attributes.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The name of segment</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A segment group, which consists of zero or more source segments, plus dimensions that are applied to those source segments. Your request can only include one segment group. Your request can include either a SegmentGroups object or a Dimensions object, but not both.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
}

/// <p>Used to create a campaign treatment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct WriteTreatmentResource {
    /// <p>The message configuration settings.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The campaign schedule.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users for this treatment.</p>
    #[serde(rename = "SizePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<i64>,
    /// <p>A custom description for the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAppError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateAppError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateAppError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateAppError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAppError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAppError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::BadRequest(ref cause) => cause,
            CreateAppError::Forbidden(ref cause) => cause,
            CreateAppError::InternalServerError(ref cause) => cause,
            CreateAppError::MethodNotAllowed(ref cause) => cause,
            CreateAppError::NotFound(ref cause) => cause,
            CreateAppError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCampaign
#[derive(Debug, PartialEq)]
pub enum CreateCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl CreateCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateCampaignError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateCampaignError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateCampaignError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateCampaignError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateCampaignError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateCampaignError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCampaignError {
    fn description(&self) -> &str {
        match *self {
            CreateCampaignError::BadRequest(ref cause) => cause,
            CreateCampaignError::Forbidden(ref cause) => cause,
            CreateCampaignError::InternalServerError(ref cause) => cause,
            CreateCampaignError::MethodNotAllowed(ref cause) => cause,
            CreateCampaignError::NotFound(ref cause) => cause,
            CreateCampaignError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateExportJob
#[derive(Debug, PartialEq)]
pub enum CreateExportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl CreateExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateExportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateExportJobError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateExportJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateExportJobError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateExportJobError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateExportJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateExportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateExportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateExportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateExportJobError::BadRequest(ref cause) => cause,
            CreateExportJobError::Forbidden(ref cause) => cause,
            CreateExportJobError::InternalServerError(ref cause) => cause,
            CreateExportJobError::MethodNotAllowed(ref cause) => cause,
            CreateExportJobError::NotFound(ref cause) => cause,
            CreateExportJobError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateImportJob
#[derive(Debug, PartialEq)]
pub enum CreateImportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl CreateImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateImportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateImportJobError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateImportJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateImportJobError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateImportJobError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateImportJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateImportJobError {
    fn description(&self) -> &str {
        match *self {
            CreateImportJobError::BadRequest(ref cause) => cause,
            CreateImportJobError::Forbidden(ref cause) => cause,
            CreateImportJobError::InternalServerError(ref cause) => cause,
            CreateImportJobError::MethodNotAllowed(ref cause) => cause,
            CreateImportJobError::NotFound(ref cause) => cause,
            CreateImportJobError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSegment
#[derive(Debug, PartialEq)]
pub enum CreateSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl CreateSegmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSegmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSegmentError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateSegmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSegmentError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateSegmentError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateSegmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateSegmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSegmentError {
    fn description(&self) -> &str {
        match *self {
            CreateSegmentError::BadRequest(ref cause) => cause,
            CreateSegmentError::Forbidden(ref cause) => cause,
            CreateSegmentError::InternalServerError(ref cause) => cause,
            CreateSegmentError::MethodNotAllowed(ref cause) => cause,
            CreateSegmentError::NotFound(ref cause) => cause,
            CreateSegmentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAdmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteAdmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAdmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAdmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteAdmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteAdmChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteAdmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAdmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAdmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteAdmChannelError::BadRequest(ref cause) => cause,
            DeleteAdmChannelError::Forbidden(ref cause) => cause,
            DeleteAdmChannelError::InternalServerError(ref cause) => cause,
            DeleteAdmChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteAdmChannelError::NotFound(ref cause) => cause,
            DeleteAdmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteApnsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApnsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApnsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApnsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteApnsChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteApnsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApnsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApnsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsChannelError::BadRequest(ref cause) => cause,
            DeleteApnsChannelError::Forbidden(ref cause) => cause,
            DeleteApnsChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsChannelError::NotFound(ref cause) => cause,
            DeleteApnsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteApnsSandboxChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApnsSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApnsSandboxChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApnsSandboxChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeleteApnsSandboxChannelError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteApnsSandboxChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApnsSandboxChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApnsSandboxChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsSandboxChannelError::BadRequest(ref cause) => cause,
            DeleteApnsSandboxChannelError::Forbidden(ref cause) => cause,
            DeleteApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsSandboxChannelError::NotFound(ref cause) => cause,
            DeleteApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteApnsVoipChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApnsVoipChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApnsVoipChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsVoipChannelError::BadRequest(ref cause) => cause,
            DeleteApnsVoipChannelError::Forbidden(ref cause) => cause,
            DeleteApnsVoipChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsVoipChannelError::NotFound(ref cause) => cause,
            DeleteApnsVoipChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteApnsVoipSandboxChannelError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApnsVoipSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApnsVoipSandboxChannelError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteApnsVoipSandboxChannelError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeleteApnsVoipSandboxChannelError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(
                        DeleteApnsVoipSandboxChannelError::MethodNotAllowed(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApnsVoipSandboxChannelError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteApnsVoipSandboxChannelError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            DeleteApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAppError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteAppError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteAppError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteAppError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAppError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAppError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::BadRequest(ref cause) => cause,
            DeleteAppError::Forbidden(ref cause) => cause,
            DeleteAppError::InternalServerError(ref cause) => cause,
            DeleteAppError::MethodNotAllowed(ref cause) => cause,
            DeleteAppError::NotFound(ref cause) => cause,
            DeleteAppError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBaiduChannel
#[derive(Debug, PartialEq)]
pub enum DeleteBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteBaiduChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBaiduChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteBaiduChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteBaiduChannelError::BadRequest(ref cause) => cause,
            DeleteBaiduChannelError::Forbidden(ref cause) => cause,
            DeleteBaiduChannelError::InternalServerError(ref cause) => cause,
            DeleteBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteBaiduChannelError::NotFound(ref cause) => cause,
            DeleteBaiduChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCampaign
#[derive(Debug, PartialEq)]
pub enum DeleteCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteCampaignError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteCampaignError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteCampaignError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteCampaignError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteCampaignError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteCampaignError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCampaignError {
    fn description(&self) -> &str {
        match *self {
            DeleteCampaignError::BadRequest(ref cause) => cause,
            DeleteCampaignError::Forbidden(ref cause) => cause,
            DeleteCampaignError::InternalServerError(ref cause) => cause,
            DeleteCampaignError::MethodNotAllowed(ref cause) => cause,
            DeleteCampaignError::NotFound(ref cause) => cause,
            DeleteCampaignError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEmailChannel
#[derive(Debug, PartialEq)]
pub enum DeleteEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteEmailChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEmailChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteEmailChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteEmailChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteEmailChannelError::BadRequest(ref cause) => cause,
            DeleteEmailChannelError::Forbidden(ref cause) => cause,
            DeleteEmailChannelError::InternalServerError(ref cause) => cause,
            DeleteEmailChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteEmailChannelError::NotFound(ref cause) => cause,
            DeleteEmailChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEndpointError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteEndpointError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteEndpointError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEndpointError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::BadRequest(ref cause) => cause,
            DeleteEndpointError::Forbidden(ref cause) => cause,
            DeleteEndpointError::InternalServerError(ref cause) => cause,
            DeleteEndpointError::MethodNotAllowed(ref cause) => cause,
            DeleteEndpointError::NotFound(ref cause) => cause,
            DeleteEndpointError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventStream
#[derive(Debug, PartialEq)]
pub enum DeleteEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteEventStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEventStreamError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEventStreamError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteEventStreamError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteEventStreamError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEventStreamError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEventStreamError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventStreamError::BadRequest(ref cause) => cause,
            DeleteEventStreamError::Forbidden(ref cause) => cause,
            DeleteEventStreamError::InternalServerError(ref cause) => cause,
            DeleteEventStreamError::MethodNotAllowed(ref cause) => cause,
            DeleteEventStreamError::NotFound(ref cause) => cause,
            DeleteEventStreamError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGcmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteGcmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGcmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGcmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteGcmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteGcmChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteGcmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGcmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGcmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteGcmChannelError::BadRequest(ref cause) => cause,
            DeleteGcmChannelError::Forbidden(ref cause) => cause,
            DeleteGcmChannelError::InternalServerError(ref cause) => cause,
            DeleteGcmChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteGcmChannelError::NotFound(ref cause) => cause,
            DeleteGcmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSegment
#[derive(Debug, PartialEq)]
pub enum DeleteSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteSegmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSegmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSegmentError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteSegmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteSegmentError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteSegmentError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSegmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteSegmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSegmentError {
    fn description(&self) -> &str {
        match *self {
            DeleteSegmentError::BadRequest(ref cause) => cause,
            DeleteSegmentError::Forbidden(ref cause) => cause,
            DeleteSegmentError::InternalServerError(ref cause) => cause,
            DeleteSegmentError::MethodNotAllowed(ref cause) => cause,
            DeleteSegmentError::NotFound(ref cause) => cause,
            DeleteSegmentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSmsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteSmsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSmsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSmsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteSmsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteSmsChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteSmsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSmsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteSmsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteSmsChannelError::BadRequest(ref cause) => cause,
            DeleteSmsChannelError::Forbidden(ref cause) => cause,
            DeleteSmsChannelError::InternalServerError(ref cause) => cause,
            DeleteSmsChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteSmsChannelError::NotFound(ref cause) => cause,
            DeleteSmsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUserEndpoints
#[derive(Debug, PartialEq)]
pub enum DeleteUserEndpointsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteUserEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUserEndpointsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUserEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUserEndpointsError {
    fn description(&self) -> &str {
        match *self {
            DeleteUserEndpointsError::BadRequest(ref cause) => cause,
            DeleteUserEndpointsError::Forbidden(ref cause) => cause,
            DeleteUserEndpointsError::InternalServerError(ref cause) => cause,
            DeleteUserEndpointsError::MethodNotAllowed(ref cause) => cause,
            DeleteUserEndpointsError::NotFound(ref cause) => cause,
            DeleteUserEndpointsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVoiceChannel
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl DeleteVoiceChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVoiceChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVoiceChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVoiceChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVoiceChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteVoiceChannelError::BadRequest(ref cause) => cause,
            DeleteVoiceChannelError::Forbidden(ref cause) => cause,
            DeleteVoiceChannelError::InternalServerError(ref cause) => cause,
            DeleteVoiceChannelError::MethodNotAllowed(ref cause) => cause,
            DeleteVoiceChannelError::NotFound(ref cause) => cause,
            DeleteVoiceChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAdmChannel
#[derive(Debug, PartialEq)]
pub enum GetAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetAdmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAdmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAdmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAdmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetAdmChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetAdmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAdmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAdmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            GetAdmChannelError::BadRequest(ref cause) => cause,
            GetAdmChannelError::Forbidden(ref cause) => cause,
            GetAdmChannelError::InternalServerError(ref cause) => cause,
            GetAdmChannelError::MethodNotAllowed(ref cause) => cause,
            GetAdmChannelError::NotFound(ref cause) => cause,
            GetAdmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetApnsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApnsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApnsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApnsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApnsChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApnsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApnsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApnsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsChannelError::BadRequest(ref cause) => cause,
            GetApnsChannelError::Forbidden(ref cause) => cause,
            GetApnsChannelError::InternalServerError(ref cause) => cause,
            GetApnsChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsChannelError::NotFound(ref cause) => cause,
            GetApnsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetApnsSandboxChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApnsSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApnsSandboxChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsSandboxChannelError::BadRequest(ref cause) => cause,
            GetApnsSandboxChannelError::Forbidden(ref cause) => cause,
            GetApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            GetApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsSandboxChannelError::NotFound(ref cause) => cause,
            GetApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetApnsVoipChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApnsVoipChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApnsVoipChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsVoipChannelError::BadRequest(ref cause) => cause,
            GetApnsVoipChannelError::Forbidden(ref cause) => cause,
            GetApnsVoipChannelError::InternalServerError(ref cause) => cause,
            GetApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsVoipChannelError::NotFound(ref cause) => cause,
            GetApnsVoipChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetApnsVoipSandboxChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApnsVoipSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApnsVoipSandboxChannelError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApnsVoipSandboxChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetApnsVoipSandboxChannelError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApnsVoipSandboxChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApnsVoipSandboxChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApnsVoipSandboxChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            GetApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            GetApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            GetApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            GetApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            GetApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            GetApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAppError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAppError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetAppError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetAppError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetAppError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAppError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::BadRequest(ref cause) => cause,
            GetAppError::Forbidden(ref cause) => cause,
            GetAppError::InternalServerError(ref cause) => cause,
            GetAppError::MethodNotAllowed(ref cause) => cause,
            GetAppError::NotFound(ref cause) => cause,
            GetAppError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplicationSettings
#[derive(Debug, PartialEq)]
pub enum GetApplicationSettingsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetApplicationSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApplicationSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationSettingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationSettingsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetApplicationSettingsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApplicationSettingsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationSettingsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationSettingsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationSettingsError::BadRequest(ref cause) => cause,
            GetApplicationSettingsError::Forbidden(ref cause) => cause,
            GetApplicationSettingsError::InternalServerError(ref cause) => cause,
            GetApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            GetApplicationSettingsError::NotFound(ref cause) => cause,
            GetApplicationSettingsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApps
#[derive(Debug, PartialEq)]
pub enum GetAppsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAppsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetAppsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetAppsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetAppsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAppsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAppsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppsError {
    fn description(&self) -> &str {
        match *self {
            GetAppsError::BadRequest(ref cause) => cause,
            GetAppsError::Forbidden(ref cause) => cause,
            GetAppsError::InternalServerError(ref cause) => cause,
            GetAppsError::MethodNotAllowed(ref cause) => cause,
            GetAppsError::NotFound(ref cause) => cause,
            GetAppsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBaiduChannel
#[derive(Debug, PartialEq)]
pub enum GetBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetBaiduChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBaiduChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBaiduChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetBaiduChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetBaiduChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetBaiduChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBaiduChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetBaiduChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            GetBaiduChannelError::BadRequest(ref cause) => cause,
            GetBaiduChannelError::Forbidden(ref cause) => cause,
            GetBaiduChannelError::InternalServerError(ref cause) => cause,
            GetBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            GetBaiduChannelError::NotFound(ref cause) => cause,
            GetBaiduChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaign
#[derive(Debug, PartialEq)]
pub enum GetCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignError::BadRequest(ref cause) => cause,
            GetCampaignError::Forbidden(ref cause) => cause,
            GetCampaignError::InternalServerError(ref cause) => cause,
            GetCampaignError::MethodNotAllowed(ref cause) => cause,
            GetCampaignError::NotFound(ref cause) => cause,
            GetCampaignError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignActivities
#[derive(Debug, PartialEq)]
pub enum GetCampaignActivitiesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetCampaignActivitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignActivitiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignActivitiesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCampaignActivitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignActivitiesError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignActivitiesError::BadRequest(ref cause) => cause,
            GetCampaignActivitiesError::Forbidden(ref cause) => cause,
            GetCampaignActivitiesError::InternalServerError(ref cause) => cause,
            GetCampaignActivitiesError::MethodNotAllowed(ref cause) => cause,
            GetCampaignActivitiesError::NotFound(ref cause) => cause,
            GetCampaignActivitiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignVersion
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetCampaignVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignVersionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignVersionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignVersionError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCampaignVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionError::BadRequest(ref cause) => cause,
            GetCampaignVersionError::Forbidden(ref cause) => cause,
            GetCampaignVersionError::InternalServerError(ref cause) => cause,
            GetCampaignVersionError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionError::NotFound(ref cause) => cause,
            GetCampaignVersionError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaignVersions
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetCampaignVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignVersionsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignVersionsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignVersionsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCampaignVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignVersionsError::BadRequest(ref cause) => cause,
            GetCampaignVersionsError::Forbidden(ref cause) => cause,
            GetCampaignVersionsError::InternalServerError(ref cause) => cause,
            GetCampaignVersionsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignVersionsError::NotFound(ref cause) => cause,
            GetCampaignVersionsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCampaigns
#[derive(Debug, PartialEq)]
pub enum GetCampaignsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetCampaignsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCampaignsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCampaignsError {
    fn description(&self) -> &str {
        match *self {
            GetCampaignsError::BadRequest(ref cause) => cause,
            GetCampaignsError::Forbidden(ref cause) => cause,
            GetCampaignsError::InternalServerError(ref cause) => cause,
            GetCampaignsError::MethodNotAllowed(ref cause) => cause,
            GetCampaignsError::NotFound(ref cause) => cause,
            GetCampaignsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetChannels
#[derive(Debug, PartialEq)]
pub enum GetChannelsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetChannelsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetChannelsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetChannelsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetChannelsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetChannelsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetChannelsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetChannelsError {
    fn description(&self) -> &str {
        match *self {
            GetChannelsError::BadRequest(ref cause) => cause,
            GetChannelsError::Forbidden(ref cause) => cause,
            GetChannelsError::InternalServerError(ref cause) => cause,
            GetChannelsError::MethodNotAllowed(ref cause) => cause,
            GetChannelsError::NotFound(ref cause) => cause,
            GetChannelsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEmailChannel
#[derive(Debug, PartialEq)]
pub enum GetEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetEmailChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetEmailChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetEmailChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetEmailChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            GetEmailChannelError::BadRequest(ref cause) => cause,
            GetEmailChannelError::Forbidden(ref cause) => cause,
            GetEmailChannelError::InternalServerError(ref cause) => cause,
            GetEmailChannelError::MethodNotAllowed(ref cause) => cause,
            GetEmailChannelError::NotFound(ref cause) => cause,
            GetEmailChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEndpoint
#[derive(Debug, PartialEq)]
pub enum GetEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEndpointError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetEndpointError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetEndpointError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEndpointError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEndpointError {
    fn description(&self) -> &str {
        match *self {
            GetEndpointError::BadRequest(ref cause) => cause,
            GetEndpointError::Forbidden(ref cause) => cause,
            GetEndpointError::InternalServerError(ref cause) => cause,
            GetEndpointError::MethodNotAllowed(ref cause) => cause,
            GetEndpointError::NotFound(ref cause) => cause,
            GetEndpointError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEventStream
#[derive(Debug, PartialEq)]
pub enum GetEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetEventStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEventStreamError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetEventStreamError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetEventStreamError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetEventStreamError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEventStreamError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEventStreamError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEventStreamError {
    fn description(&self) -> &str {
        match *self {
            GetEventStreamError::BadRequest(ref cause) => cause,
            GetEventStreamError::Forbidden(ref cause) => cause,
            GetEventStreamError::InternalServerError(ref cause) => cause,
            GetEventStreamError::MethodNotAllowed(ref cause) => cause,
            GetEventStreamError::NotFound(ref cause) => cause,
            GetEventStreamError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExportJob
#[derive(Debug, PartialEq)]
pub enum GetExportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetExportJobError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetExportJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetExportJobError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetExportJobError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetExportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetExportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportJobError {
    fn description(&self) -> &str {
        match *self {
            GetExportJobError::BadRequest(ref cause) => cause,
            GetExportJobError::Forbidden(ref cause) => cause,
            GetExportJobError::InternalServerError(ref cause) => cause,
            GetExportJobError::MethodNotAllowed(ref cause) => cause,
            GetExportJobError::NotFound(ref cause) => cause,
            GetExportJobError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExportJobs
#[derive(Debug, PartialEq)]
pub enum GetExportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetExportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetExportJobsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetExportJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetExportJobsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetExportJobsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetExportJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetExportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetExportJobsError::BadRequest(ref cause) => cause,
            GetExportJobsError::Forbidden(ref cause) => cause,
            GetExportJobsError::InternalServerError(ref cause) => cause,
            GetExportJobsError::MethodNotAllowed(ref cause) => cause,
            GetExportJobsError::NotFound(ref cause) => cause,
            GetExportJobsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGcmChannel
#[derive(Debug, PartialEq)]
pub enum GetGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetGcmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGcmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGcmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetGcmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetGcmChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetGcmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGcmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGcmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            GetGcmChannelError::BadRequest(ref cause) => cause,
            GetGcmChannelError::Forbidden(ref cause) => cause,
            GetGcmChannelError::InternalServerError(ref cause) => cause,
            GetGcmChannelError::MethodNotAllowed(ref cause) => cause,
            GetGcmChannelError::NotFound(ref cause) => cause,
            GetGcmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetImportJob
#[derive(Debug, PartialEq)]
pub enum GetImportJobError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImportJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetImportJobError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImportJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetImportJobError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetImportJobError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetImportJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetImportJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetImportJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobError::BadRequest(ref cause) => cause,
            GetImportJobError::Forbidden(ref cause) => cause,
            GetImportJobError::InternalServerError(ref cause) => cause,
            GetImportJobError::MethodNotAllowed(ref cause) => cause,
            GetImportJobError::NotFound(ref cause) => cause,
            GetImportJobError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetImportJobs
#[derive(Debug, PartialEq)]
pub enum GetImportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetImportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetImportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetImportJobsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetImportJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetImportJobsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetImportJobsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetImportJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetImportJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetImportJobsError::BadRequest(ref cause) => cause,
            GetImportJobsError::Forbidden(ref cause) => cause,
            GetImportJobsError::InternalServerError(ref cause) => cause,
            GetImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetImportJobsError::NotFound(ref cause) => cause,
            GetImportJobsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegment
#[derive(Debug, PartialEq)]
pub enum GetSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentError::BadRequest(ref cause) => cause,
            GetSegmentError::Forbidden(ref cause) => cause,
            GetSegmentError::InternalServerError(ref cause) => cause,
            GetSegmentError::MethodNotAllowed(ref cause) => cause,
            GetSegmentError::NotFound(ref cause) => cause,
            GetSegmentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentExportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentExportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentExportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentExportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentExportJobsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentExportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentExportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentExportJobsError::BadRequest(ref cause) => cause,
            GetSegmentExportJobsError::Forbidden(ref cause) => cause,
            GetSegmentExportJobsError::InternalServerError(ref cause) => cause,
            GetSegmentExportJobsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentExportJobsError::NotFound(ref cause) => cause,
            GetSegmentExportJobsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentImportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentImportJobsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentImportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentImportJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentImportJobsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentImportJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentImportJobsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentImportJobsError::BadRequest(ref cause) => cause,
            GetSegmentImportJobsError::Forbidden(ref cause) => cause,
            GetSegmentImportJobsError::InternalServerError(ref cause) => cause,
            GetSegmentImportJobsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentImportJobsError::NotFound(ref cause) => cause,
            GetSegmentImportJobsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentVersion
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentVersionError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentVersionError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentVersionError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionError::BadRequest(ref cause) => cause,
            GetSegmentVersionError::Forbidden(ref cause) => cause,
            GetSegmentVersionError::InternalServerError(ref cause) => cause,
            GetSegmentVersionError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionError::NotFound(ref cause) => cause,
            GetSegmentVersionError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegmentVersions
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentVersionsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentVersionsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentVersionsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentVersionsError::BadRequest(ref cause) => cause,
            GetSegmentVersionsError::Forbidden(ref cause) => cause,
            GetSegmentVersionsError::InternalServerError(ref cause) => cause,
            GetSegmentVersionsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentVersionsError::NotFound(ref cause) => cause,
            GetSegmentVersionsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSegments
#[derive(Debug, PartialEq)]
pub enum GetSegmentsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSegmentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSegmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSegmentsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSegmentsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSegmentsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSegmentsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSegmentsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSegmentsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSegmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSegmentsError {
    fn description(&self) -> &str {
        match *self {
            GetSegmentsError::BadRequest(ref cause) => cause,
            GetSegmentsError::Forbidden(ref cause) => cause,
            GetSegmentsError::InternalServerError(ref cause) => cause,
            GetSegmentsError::MethodNotAllowed(ref cause) => cause,
            GetSegmentsError::NotFound(ref cause) => cause,
            GetSegmentsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSmsChannel
#[derive(Debug, PartialEq)]
pub enum GetSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetSmsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSmsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSmsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSmsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSmsChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSmsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSmsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSmsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            GetSmsChannelError::BadRequest(ref cause) => cause,
            GetSmsChannelError::Forbidden(ref cause) => cause,
            GetSmsChannelError::InternalServerError(ref cause) => cause,
            GetSmsChannelError::MethodNotAllowed(ref cause) => cause,
            GetSmsChannelError::NotFound(ref cause) => cause,
            GetSmsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUserEndpoints
#[derive(Debug, PartialEq)]
pub enum GetUserEndpointsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetUserEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUserEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUserEndpointsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetUserEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetUserEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetUserEndpointsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUserEndpointsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUserEndpointsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUserEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUserEndpointsError {
    fn description(&self) -> &str {
        match *self {
            GetUserEndpointsError::BadRequest(ref cause) => cause,
            GetUserEndpointsError::Forbidden(ref cause) => cause,
            GetUserEndpointsError::InternalServerError(ref cause) => cause,
            GetUserEndpointsError::MethodNotAllowed(ref cause) => cause,
            GetUserEndpointsError::NotFound(ref cause) => cause,
            GetUserEndpointsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVoiceChannel
#[derive(Debug, PartialEq)]
pub enum GetVoiceChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl GetVoiceChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVoiceChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetVoiceChannelError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetVoiceChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetVoiceChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVoiceChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVoiceChannelError {
    fn description(&self) -> &str {
        match *self {
            GetVoiceChannelError::BadRequest(ref cause) => cause,
            GetVoiceChannelError::Forbidden(ref cause) => cause,
            GetVoiceChannelError::InternalServerError(ref cause) => cause,
            GetVoiceChannelError::MethodNotAllowed(ref cause) => cause,
            GetVoiceChannelError::NotFound(ref cause) => cause,
            GetVoiceChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by PhoneNumberValidate
#[derive(Debug, PartialEq)]
pub enum PhoneNumberValidateError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl PhoneNumberValidateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PhoneNumberValidateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PhoneNumberValidateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PhoneNumberValidateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PhoneNumberValidateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(PhoneNumberValidateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PhoneNumberValidateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PhoneNumberValidateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PhoneNumberValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PhoneNumberValidateError {
    fn description(&self) -> &str {
        match *self {
            PhoneNumberValidateError::BadRequest(ref cause) => cause,
            PhoneNumberValidateError::Forbidden(ref cause) => cause,
            PhoneNumberValidateError::InternalServerError(ref cause) => cause,
            PhoneNumberValidateError::MethodNotAllowed(ref cause) => cause,
            PhoneNumberValidateError::NotFound(ref cause) => cause,
            PhoneNumberValidateError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEventStream
#[derive(Debug, PartialEq)]
pub enum PutEventStreamError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl PutEventStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventStreamError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutEventStreamError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutEventStreamError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutEventStreamError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(PutEventStreamError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEventStreamError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutEventStreamError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventStreamError {
    fn description(&self) -> &str {
        match *self {
            PutEventStreamError::BadRequest(ref cause) => cause,
            PutEventStreamError::Forbidden(ref cause) => cause,
            PutEventStreamError::InternalServerError(ref cause) => cause,
            PutEventStreamError::MethodNotAllowed(ref cause) => cause,
            PutEventStreamError::NotFound(ref cause) => cause,
            PutEventStreamError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl PutEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutEventsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PutEventsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PutEventsError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(PutEventsError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEventsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutEventsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventsError {
    fn description(&self) -> &str {
        match *self {
            PutEventsError::BadRequest(ref cause) => cause,
            PutEventsError::Forbidden(ref cause) => cause,
            PutEventsError::InternalServerError(ref cause) => cause,
            PutEventsError::MethodNotAllowed(ref cause) => cause,
            PutEventsError::NotFound(ref cause) => cause,
            PutEventsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAttributes
#[derive(Debug, PartialEq)]
pub enum RemoveAttributesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl RemoveAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(RemoveAttributesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RemoveAttributesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(RemoveAttributesError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(RemoveAttributesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RemoveAttributesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemoveAttributesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAttributesError {
    fn description(&self) -> &str {
        match *self {
            RemoveAttributesError::BadRequest(ref cause) => cause,
            RemoveAttributesError::Forbidden(ref cause) => cause,
            RemoveAttributesError::InternalServerError(ref cause) => cause,
            RemoveAttributesError::MethodNotAllowed(ref cause) => cause,
            RemoveAttributesError::NotFound(ref cause) => cause,
            RemoveAttributesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SendMessages
#[derive(Debug, PartialEq)]
pub enum SendMessagesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl SendMessagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendMessagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SendMessagesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(SendMessagesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(SendMessagesError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(SendMessagesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendMessagesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendMessagesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendMessagesError {
    fn description(&self) -> &str {
        match *self {
            SendMessagesError::BadRequest(ref cause) => cause,
            SendMessagesError::Forbidden(ref cause) => cause,
            SendMessagesError::InternalServerError(ref cause) => cause,
            SendMessagesError::MethodNotAllowed(ref cause) => cause,
            SendMessagesError::NotFound(ref cause) => cause,
            SendMessagesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SendUsersMessages
#[derive(Debug, PartialEq)]
pub enum SendUsersMessagesError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl SendUsersMessagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendUsersMessagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SendUsersMessagesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(SendUsersMessagesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(SendUsersMessagesError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(SendUsersMessagesError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendUsersMessagesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendUsersMessagesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendUsersMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendUsersMessagesError {
    fn description(&self) -> &str {
        match *self {
            SendUsersMessagesError::BadRequest(ref cause) => cause,
            SendUsersMessagesError::Forbidden(ref cause) => cause,
            SendUsersMessagesError::InternalServerError(ref cause) => cause,
            SendUsersMessagesError::MethodNotAllowed(ref cause) => cause,
            SendUsersMessagesError::NotFound(ref cause) => cause,
            SendUsersMessagesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAdmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateAdmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateAdmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAdmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAdmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateAdmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateAdmChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateAdmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAdmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAdmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAdmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAdmChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateAdmChannelError::BadRequest(ref cause) => cause,
            UpdateAdmChannelError::Forbidden(ref cause) => cause,
            UpdateAdmChannelError::InternalServerError(ref cause) => cause,
            UpdateAdmChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateAdmChannelError::NotFound(ref cause) => cause,
            UpdateAdmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateApnsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApnsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApnsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApnsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateApnsChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateApnsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApnsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApnsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApnsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsChannelError::BadRequest(ref cause) => cause,
            UpdateApnsChannelError::Forbidden(ref cause) => cause,
            UpdateApnsChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsChannelError::NotFound(ref cause) => cause,
            UpdateApnsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateApnsSandboxChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApnsSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApnsSandboxChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApnsSandboxChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateApnsSandboxChannelError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateApnsSandboxChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApnsSandboxChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApnsSandboxChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApnsSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsSandboxChannelError::BadRequest(ref cause) => cause,
            UpdateApnsSandboxChannelError::Forbidden(ref cause) => cause,
            UpdateApnsSandboxChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsSandboxChannelError::NotFound(ref cause) => cause,
            UpdateApnsSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateApnsVoipChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApnsVoipChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApnsVoipChannelError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApnsVoipChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsVoipChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsVoipChannelError::BadRequest(ref cause) => cause,
            UpdateApnsVoipChannelError::Forbidden(ref cause) => cause,
            UpdateApnsVoipChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsVoipChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsVoipChannelError::NotFound(ref cause) => cause,
            UpdateApnsVoipChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipSandboxChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateApnsVoipSandboxChannelError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateApnsVoipSandboxChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApnsVoipSandboxChannelError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApnsVoipSandboxChannelError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateApnsVoipSandboxChannelError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(
                        UpdateApnsVoipSandboxChannelError::MethodNotAllowed(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApnsVoipSandboxChannelError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateApnsVoipSandboxChannelError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApnsVoipSandboxChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApnsVoipSandboxChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateApnsVoipSandboxChannelError::BadRequest(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::Forbidden(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::InternalServerError(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::NotFound(ref cause) => cause,
            UpdateApnsVoipSandboxChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplicationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationSettingsError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateApplicationSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApplicationSettingsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateApplicationSettingsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateApplicationSettingsError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateApplicationSettingsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApplicationSettingsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApplicationSettingsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApplicationSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationSettingsError::BadRequest(ref cause) => cause,
            UpdateApplicationSettingsError::Forbidden(ref cause) => cause,
            UpdateApplicationSettingsError::InternalServerError(ref cause) => cause,
            UpdateApplicationSettingsError::MethodNotAllowed(ref cause) => cause,
            UpdateApplicationSettingsError::NotFound(ref cause) => cause,
            UpdateApplicationSettingsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBaiduChannel
#[derive(Debug, PartialEq)]
pub enum UpdateBaiduChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateBaiduChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBaiduChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateBaiduChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateBaiduChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBaiduChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateBaiduChannelError::BadRequest(ref cause) => cause,
            UpdateBaiduChannelError::Forbidden(ref cause) => cause,
            UpdateBaiduChannelError::InternalServerError(ref cause) => cause,
            UpdateBaiduChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateBaiduChannelError::NotFound(ref cause) => cause,
            UpdateBaiduChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCampaign
#[derive(Debug, PartialEq)]
pub enum UpdateCampaignError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateCampaignError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateCampaignError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateCampaignError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateCampaignError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateCampaignError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateCampaignError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCampaignError {
    fn description(&self) -> &str {
        match *self {
            UpdateCampaignError::BadRequest(ref cause) => cause,
            UpdateCampaignError::Forbidden(ref cause) => cause,
            UpdateCampaignError::InternalServerError(ref cause) => cause,
            UpdateCampaignError::MethodNotAllowed(ref cause) => cause,
            UpdateCampaignError::NotFound(ref cause) => cause,
            UpdateCampaignError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEmailChannel
#[derive(Debug, PartialEq)]
pub enum UpdateEmailChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateEmailChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEmailChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEmailChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEmailChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateEmailChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateEmailChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEmailChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEmailChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateEmailChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEmailChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateEmailChannelError::BadRequest(ref cause) => cause,
            UpdateEmailChannelError::Forbidden(ref cause) => cause,
            UpdateEmailChannelError::InternalServerError(ref cause) => cause,
            UpdateEmailChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateEmailChannelError::NotFound(ref cause) => cause,
            UpdateEmailChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEndpointError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEndpointError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateEndpointError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateEndpointError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEndpointError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointError::BadRequest(ref cause) => cause,
            UpdateEndpointError::Forbidden(ref cause) => cause,
            UpdateEndpointError::InternalServerError(ref cause) => cause,
            UpdateEndpointError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointError::NotFound(ref cause) => cause,
            UpdateEndpointError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEndpointsBatch
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointsBatchError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateEndpointsBatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEndpointsBatchError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEndpointsBatchError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateEndpointsBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEndpointsBatchError {
    fn description(&self) -> &str {
        match *self {
            UpdateEndpointsBatchError::BadRequest(ref cause) => cause,
            UpdateEndpointsBatchError::Forbidden(ref cause) => cause,
            UpdateEndpointsBatchError::InternalServerError(ref cause) => cause,
            UpdateEndpointsBatchError::MethodNotAllowed(ref cause) => cause,
            UpdateEndpointsBatchError::NotFound(ref cause) => cause,
            UpdateEndpointsBatchError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGcmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateGcmChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateGcmChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGcmChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGcmChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateGcmChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateGcmChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateGcmChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGcmChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGcmChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGcmChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGcmChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateGcmChannelError::BadRequest(ref cause) => cause,
            UpdateGcmChannelError::Forbidden(ref cause) => cause,
            UpdateGcmChannelError::InternalServerError(ref cause) => cause,
            UpdateGcmChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateGcmChannelError::NotFound(ref cause) => cause,
            UpdateGcmChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSegment
#[derive(Debug, PartialEq)]
pub enum UpdateSegmentError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateSegmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSegmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateSegmentError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateSegmentError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateSegmentError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateSegmentError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSegmentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateSegmentError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSegmentError {
    fn description(&self) -> &str {
        match *self {
            UpdateSegmentError::BadRequest(ref cause) => cause,
            UpdateSegmentError::Forbidden(ref cause) => cause,
            UpdateSegmentError::InternalServerError(ref cause) => cause,
            UpdateSegmentError::MethodNotAllowed(ref cause) => cause,
            UpdateSegmentError::NotFound(ref cause) => cause,
            UpdateSegmentError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSmsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateSmsChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateSmsChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSmsChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateSmsChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateSmsChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateSmsChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateSmsChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSmsChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateSmsChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateSmsChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSmsChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateSmsChannelError::BadRequest(ref cause) => cause,
            UpdateSmsChannelError::Forbidden(ref cause) => cause,
            UpdateSmsChannelError::InternalServerError(ref cause) => cause,
            UpdateSmsChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateSmsChannelError::NotFound(ref cause) => cause,
            UpdateSmsChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVoiceChannel
#[derive(Debug, PartialEq)]
pub enum UpdateVoiceChannelError {
    /// <p>Simple message object.</p>
    BadRequest(String),
    /// <p>Simple message object.</p>
    Forbidden(String),
    /// <p>Simple message object.</p>
    InternalServerError(String),
    /// <p>Simple message object.</p>
    MethodNotAllowed(String),
    /// <p>Simple message object.</p>
    NotFound(String),
    /// <p>Simple message object.</p>
    TooManyRequests(String),
}

impl UpdateVoiceChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVoiceChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVoiceChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateVoiceChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVoiceChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateVoiceChannelError::BadRequest(ref cause) => cause,
            UpdateVoiceChannelError::Forbidden(ref cause) => cause,
            UpdateVoiceChannelError::InternalServerError(ref cause) => cause,
            UpdateVoiceChannelError::MethodNotAllowed(ref cause) => cause,
            UpdateVoiceChannelError::NotFound(ref cause) => cause,
            UpdateVoiceChannelError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Pinpoint API. Amazon Pinpoint clients implement this trait.
pub trait Pinpoint {
    /// <p>Creates or updates an app.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError>;

    /// <p>Creates or updates a campaign.</p>
    fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> RusotoFuture<CreateCampaignResponse, CreateCampaignError>;

    /// <p>Creates an export job.</p>
    fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> RusotoFuture<CreateExportJobResponse, CreateExportJobError>;

    /// <p>Creates or updates an import job.</p>
    fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> RusotoFuture<CreateImportJobResponse, CreateImportJobError>;

    /// <p>Used to create or update a segment.</p>
    fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> RusotoFuture<CreateSegmentResponse, CreateSegmentError>;

    /// <p>Delete an ADM channel.</p>
    fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> RusotoFuture<DeleteAdmChannelResponse, DeleteAdmChannelError>;

    /// <p>Deletes the APNs channel for an app.</p>
    fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> RusotoFuture<DeleteApnsChannelResponse, DeleteApnsChannelError>;

    /// <p>Delete an APNS sandbox channel.</p>
    fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError>;

    /// <p>Delete an APNS VoIP channel</p>
    fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipChannelResponse, DeleteApnsVoipChannelError>;

    /// <p>Delete an APNS VoIP sandbox channel</p>
    fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipSandboxChannelResponse, DeleteApnsVoipSandboxChannelError>;

    /// <p>Deletes an app.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError>;

    /// <p>Delete a BAIDU GCM channel</p>
    fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> RusotoFuture<DeleteBaiduChannelResponse, DeleteBaiduChannelError>;

    /// <p>Deletes a campaign.</p>
    fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> RusotoFuture<DeleteCampaignResponse, DeleteCampaignError>;

    /// <p>Delete an email channel.</p>
    fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> RusotoFuture<DeleteEmailChannelResponse, DeleteEmailChannelError>;

    /// <p>Deletes an endpoint.</p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError>;

    /// <p>Deletes the event stream for an app.</p>
    fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> RusotoFuture<DeleteEventStreamResponse, DeleteEventStreamError>;

    /// <p>Deletes the GCM channel for an app.</p>
    fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> RusotoFuture<DeleteGcmChannelResponse, DeleteGcmChannelError>;

    /// <p>Deletes a segment.</p>
    fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> RusotoFuture<DeleteSegmentResponse, DeleteSegmentError>;

    /// <p>Delete an SMS channel.</p>
    fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> RusotoFuture<DeleteSmsChannelResponse, DeleteSmsChannelError>;

    /// <p>Deletes endpoints that are associated with a User ID.</p>
    fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> RusotoFuture<DeleteUserEndpointsResponse, DeleteUserEndpointsError>;

    /// <p>Delete an Voice channel</p>
    fn delete_voice_channel(
        &self,
        input: DeleteVoiceChannelRequest,
    ) -> RusotoFuture<DeleteVoiceChannelResponse, DeleteVoiceChannelError>;

    /// <p>Get an ADM channel.</p>
    fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> RusotoFuture<GetAdmChannelResponse, GetAdmChannelError>;

    /// <p>Returns information about the APNs channel for an app.</p>
    fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> RusotoFuture<GetApnsChannelResponse, GetApnsChannelError>;

    /// <p>Get an APNS sandbox channel.</p>
    fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError>;

    /// <p>Get an APNS VoIP channel</p>
    fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> RusotoFuture<GetApnsVoipChannelResponse, GetApnsVoipChannelError>;

    /// <p>Get an APNS VoIPSandbox channel</p>
    fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsVoipSandboxChannelResponse, GetApnsVoipSandboxChannelError>;

    /// <p>Returns information about an app.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError>;

    /// <p>Used to request the settings for an app.</p>
    fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> RusotoFuture<GetApplicationSettingsResponse, GetApplicationSettingsError>;

    /// <p>Returns information about your apps.</p>
    fn get_apps(&self, input: GetAppsRequest) -> RusotoFuture<GetAppsResponse, GetAppsError>;

    /// <p>Get a BAIDU GCM channel</p>
    fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> RusotoFuture<GetBaiduChannelResponse, GetBaiduChannelError>;

    /// <p>Returns information about a campaign.</p>
    fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> RusotoFuture<GetCampaignResponse, GetCampaignError>;

    /// <p>Returns information about the activity performed by a campaign.</p>
    fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> RusotoFuture<GetCampaignActivitiesResponse, GetCampaignActivitiesError>;

    /// <p>Returns information about a specific version of a campaign.</p>
    fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> RusotoFuture<GetCampaignVersionResponse, GetCampaignVersionError>;

    /// <p>Returns information about your campaign versions.</p>
    fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> RusotoFuture<GetCampaignVersionsResponse, GetCampaignVersionsError>;

    /// <p>Returns information about your campaigns.</p>
    fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> RusotoFuture<GetCampaignsResponse, GetCampaignsError>;

    /// <p>Get all channels.</p>
    fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> RusotoFuture<GetChannelsResponse, GetChannelsError>;

    /// <p>Get an email channel.</p>
    fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> RusotoFuture<GetEmailChannelResponse, GetEmailChannelError>;

    /// <p>Returns information about an endpoint.</p>
    fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> RusotoFuture<GetEndpointResponse, GetEndpointError>;

    /// <p>Returns the event stream for an app.</p>
    fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> RusotoFuture<GetEventStreamResponse, GetEventStreamError>;

    /// <p>Returns information about an export job.</p>
    fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> RusotoFuture<GetExportJobResponse, GetExportJobError>;

    /// <p>Returns information about your export jobs.</p>
    fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> RusotoFuture<GetExportJobsResponse, GetExportJobsError>;

    /// <p>Returns information about the GCM channel for an app.</p>
    fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> RusotoFuture<GetGcmChannelResponse, GetGcmChannelError>;

    /// <p>Returns information about an import job.</p>
    fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> RusotoFuture<GetImportJobResponse, GetImportJobError>;

    /// <p>Returns information about your import jobs.</p>
    fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> RusotoFuture<GetImportJobsResponse, GetImportJobsError>;

    /// <p>Returns information about a segment.</p>
    fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> RusotoFuture<GetSegmentResponse, GetSegmentError>;

    /// <p>Returns a list of export jobs for a specific segment.</p>
    fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> RusotoFuture<GetSegmentExportJobsResponse, GetSegmentExportJobsError>;

    /// <p>Returns a list of import jobs for a specific segment.</p>
    fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> RusotoFuture<GetSegmentImportJobsResponse, GetSegmentImportJobsError>;

    /// <p>Returns information about a segment version.</p>
    fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> RusotoFuture<GetSegmentVersionResponse, GetSegmentVersionError>;

    /// <p>Returns information about your segment versions.</p>
    fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> RusotoFuture<GetSegmentVersionsResponse, GetSegmentVersionsError>;

    /// <p>Used to get information about your segments.</p>
    fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> RusotoFuture<GetSegmentsResponse, GetSegmentsError>;

    /// <p>Get an SMS channel.</p>
    fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> RusotoFuture<GetSmsChannelResponse, GetSmsChannelError>;

    /// <p>Returns information about the endpoints that are associated with a User ID.</p>
    fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> RusotoFuture<GetUserEndpointsResponse, GetUserEndpointsError>;

    /// <p>Get a Voice Channel</p>
    fn get_voice_channel(
        &self,
        input: GetVoiceChannelRequest,
    ) -> RusotoFuture<GetVoiceChannelResponse, GetVoiceChannelError>;

    /// <p>Returns information about the specified phone number.</p>
    fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> RusotoFuture<PhoneNumberValidateResponse, PhoneNumberValidateError>;

    /// <p>Use to create or update the event stream for an app.</p>
    fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> RusotoFuture<PutEventStreamResponse, PutEventStreamError>;

    /// <p>Use to record events for endpoints. This method creates events and creates or updates the endpoints that those events are associated with.</p>
    fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> RusotoFuture<PutEventsResponse, PutEventsError>;

    /// <p>Used to remove the attributes for an app</p>
    fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> RusotoFuture<RemoveAttributesResponse, RemoveAttributesError>;

    /// <p>Used to send a direct message.</p>
    fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> RusotoFuture<SendMessagesResponse, SendMessagesError>;

    /// <p>Used to send a message to a list of users.</p>
    fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> RusotoFuture<SendUsersMessagesResponse, SendUsersMessagesError>;

    /// <p>Update an ADM channel.</p>
    fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> RusotoFuture<UpdateAdmChannelResponse, UpdateAdmChannelError>;

    /// <p>Use to update the APNs channel for an app.</p>
    fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> RusotoFuture<UpdateApnsChannelResponse, UpdateApnsChannelError>;

    /// <p>Update an APNS sandbox channel.</p>
    fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError>;

    /// <p>Update an APNS VoIP channel</p>
    fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipChannelResponse, UpdateApnsVoipChannelError>;

    /// <p>Update an APNS VoIP sandbox channel</p>
    fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipSandboxChannelResponse, UpdateApnsVoipSandboxChannelError>;

    /// <p>Used to update the settings for an app.</p>
    fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> RusotoFuture<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError>;

    /// <p>Update a BAIDU GCM channel</p>
    fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> RusotoFuture<UpdateBaiduChannelResponse, UpdateBaiduChannelError>;

    /// <p>Use to update a campaign.</p>
    fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> RusotoFuture<UpdateCampaignResponse, UpdateCampaignError>;

    /// <p>Update an email channel.</p>
    fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> RusotoFuture<UpdateEmailChannelResponse, UpdateEmailChannelError>;

    /// <p>Creates or updates an endpoint.</p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> RusotoFuture<UpdateEndpointResponse, UpdateEndpointError>;

    /// <p>Use to update a batch of endpoints.</p>
    fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> RusotoFuture<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError>;

    /// <p>Use to update the GCM channel for an app.</p>
    fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> RusotoFuture<UpdateGcmChannelResponse, UpdateGcmChannelError>;

    /// <p>Used to update a segment.</p>
    fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> RusotoFuture<UpdateSegmentResponse, UpdateSegmentError>;

    /// <p>Update an SMS channel.</p>
    fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> RusotoFuture<UpdateSmsChannelResponse, UpdateSmsChannelError>;

    /// <p>Update an Voice channel</p>
    fn update_voice_channel(
        &self,
        input: UpdateVoiceChannelRequest,
    ) -> RusotoFuture<UpdateVoiceChannelResponse, UpdateVoiceChannelError>;
}
/// A client for the Amazon Pinpoint API.
#[derive(Clone)]
pub struct PinpointClient {
    client: Client,
    region: region::Region,
}

impl PinpointClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PinpointClient {
        PinpointClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PinpointClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PinpointClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Pinpoint for PinpointClient {
    /// <p>Creates or updates an app.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.create_application_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAppResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates a campaign.</p>
    fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> RusotoFuture<CreateCampaignResponse, CreateCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCampaignResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an export job.</p>
    fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> RusotoFuture<CreateExportJobResponse, CreateExportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.export_job_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateExportJobResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateExportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates an import job.</p>
    fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> RusotoFuture<CreateImportJobResponse, CreateImportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.import_job_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateImportJobResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to create or update a segment.</p>
    fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> RusotoFuture<CreateSegmentResponse, CreateSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateSegmentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an ADM channel.</p>
    fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> RusotoFuture<DeleteAdmChannelResponse, DeleteAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAdmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the APNs channel for an app.</p>
    fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> RusotoFuture<DeleteApnsChannelResponse, DeleteApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApnsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an APNS sandbox channel.</p>
    fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsSandboxChannelResponse, DeleteApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApnsSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApnsSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Delete an APNS VoIP channel</p>
    fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipChannelResponse, DeleteApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApnsVoipChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteApnsVoipChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Delete an APNS VoIP sandbox channel</p>
    fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<DeleteApnsVoipSandboxChannelResponse, DeleteApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApnsVoipSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an app.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAppResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete a BAIDU GCM channel</p>
    fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> RusotoFuture<DeleteBaiduChannelResponse, DeleteBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBaiduChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a campaign.</p>
    fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> RusotoFuture<DeleteCampaignResponse, DeleteCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCampaignResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an email channel.</p>
    fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> RusotoFuture<DeleteEmailChannelResponse, DeleteEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEmailChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an endpoint.</p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the event stream for an app.</p>
    fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> RusotoFuture<DeleteEventStreamResponse, DeleteEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEventStreamResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the GCM channel for an app.</p>
    fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> RusotoFuture<DeleteGcmChannelResponse, DeleteGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteGcmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a segment.</p>
    fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> RusotoFuture<DeleteSegmentResponse, DeleteSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSegmentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Delete an SMS channel.</p>
    fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> RusotoFuture<DeleteSmsChannelResponse, DeleteSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteSmsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteSmsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes endpoints that are associated with a User ID.</p>
    fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> RusotoFuture<DeleteUserEndpointsResponse, DeleteUserEndpointsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteUserEndpointsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteUserEndpointsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Delete an Voice channel</p>
    fn delete_voice_channel(
        &self,
        input: DeleteVoiceChannelRequest,
    ) -> RusotoFuture<DeleteVoiceChannelResponse, DeleteVoiceChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteVoiceChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteVoiceChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an ADM channel.</p>
    fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> RusotoFuture<GetAdmChannelResponse, GetAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAdmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the APNs channel for an app.</p>
    fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> RusotoFuture<GetApnsChannelResponse, GetApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApnsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an APNS sandbox channel.</p>
    fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsSandboxChannelResponse, GetApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApnsSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApnsSandboxChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get an APNS VoIP channel</p>
    fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> RusotoFuture<GetApnsVoipChannelResponse, GetApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApnsVoipChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApnsVoipChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an APNS VoIPSandbox channel</p>
    fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<GetApnsVoipSandboxChannelResponse, GetApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApnsVoipSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about an app.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAppResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to request the settings for an app.</p>
    fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> RusotoFuture<GetApplicationSettingsResponse, GetApplicationSettingsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetApplicationSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetApplicationSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about your apps.</p>
    fn get_apps(&self, input: GetAppsRequest) -> RusotoFuture<GetAppsResponse, GetAppsError> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAppsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get a BAIDU GCM channel</p>
    fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> RusotoFuture<GetBaiduChannelResponse, GetBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBaiduChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a campaign.</p>
    fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> RusotoFuture<GetCampaignResponse, GetCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCampaignResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the activity performed by a campaign.</p>
    fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> RusotoFuture<GetCampaignActivitiesResponse, GetCampaignActivitiesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/activities",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCampaignActivitiesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCampaignActivitiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a specific version of a campaign.</p>
    fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> RusotoFuture<GetCampaignVersionResponse, GetCampaignVersionError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/versions/{version}",
            application_id = input.application_id,
            campaign_id = input.campaign_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCampaignVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your campaign versions.</p>
    fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> RusotoFuture<GetCampaignVersionsResponse, GetCampaignVersionsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/versions",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCampaignVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetCampaignVersionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about your campaigns.</p>
    fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> RusotoFuture<GetCampaignsResponse, GetCampaignsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCampaignsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCampaignsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get all channels.</p>
    fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> RusotoFuture<GetChannelsResponse, GetChannelsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetChannelsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetChannelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an email channel.</p>
    fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> RusotoFuture<GetEmailChannelResponse, GetEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetEmailChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an endpoint.</p>
    fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> RusotoFuture<GetEndpointResponse, GetEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the event stream for an app.</p>
    fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> RusotoFuture<GetEventStreamResponse, GetEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetEventStreamResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an export job.</p>
    fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> RusotoFuture<GetExportJobResponse, GetExportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetExportJobResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your export jobs.</p>
    fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> RusotoFuture<GetExportJobsResponse, GetExportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetExportJobsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the GCM channel for an app.</p>
    fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> RusotoFuture<GetGcmChannelResponse, GetGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetGcmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about an import job.</p>
    fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> RusotoFuture<GetImportJobResponse, GetImportJobError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetImportJobResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetImportJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your import jobs.</p>
    fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> RusotoFuture<GetImportJobsResponse, GetImportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetImportJobsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetImportJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a segment.</p>
    fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> RusotoFuture<GetSegmentResponse, GetSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of export jobs for a specific segment.</p>
    fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> RusotoFuture<GetSegmentExportJobsResponse, GetSegmentExportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/jobs/export",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentExportJobsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSegmentExportJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of import jobs for a specific segment.</p>
    fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> RusotoFuture<GetSegmentImportJobsResponse, GetSegmentImportJobsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/jobs/import",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentImportJobsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSegmentImportJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a segment version.</p>
    fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> RusotoFuture<GetSegmentVersionResponse, GetSegmentVersionError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/versions/{version}",
            application_id = input.application_id,
            segment_id = input.segment_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentVersionResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about your segment versions.</p>
    fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> RusotoFuture<GetSegmentVersionsResponse, GetSegmentVersionsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/versions",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentVersionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to get information about your segments.</p>
    fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> RusotoFuture<GetSegmentsResponse, GetSegmentsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.token {
            params.put("token", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSegmentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSegmentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get an SMS channel.</p>
    fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> RusotoFuture<GetSmsChannelResponse, GetSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSmsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSmsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the endpoints that are associated with a User ID.</p>
    fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> RusotoFuture<GetUserEndpointsResponse, GetUserEndpointsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetUserEndpointsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUserEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get a Voice Channel</p>
    fn get_voice_channel(
        &self,
        input: GetVoiceChannelRequest,
    ) -> RusotoFuture<GetVoiceChannelResponse, GetVoiceChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetVoiceChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetVoiceChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the specified phone number.</p>
    fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> RusotoFuture<PhoneNumberValidateResponse, PhoneNumberValidateError> {
        let request_uri = "/v1/phone/number/validate";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.number_validate_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PhoneNumberValidateResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PhoneNumberValidateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use to create or update the event stream for an app.</p>
    fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> RusotoFuture<PutEventStreamResponse, PutEventStreamError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_event_stream).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutEventStreamResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutEventStreamError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to record events for endpoints. This method creates events and creates or updates the endpoints that those events are associated with.</p>
    fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> RusotoFuture<PutEventsResponse, PutEventsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/events",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.events_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutEventsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to remove the attributes for an app</p>
    fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> RusotoFuture<RemoveAttributesResponse, RemoveAttributesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/attributes/{attribute_type}",
            application_id = input.application_id,
            attribute_type = input.attribute_type
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.update_attributes_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveAttributesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RemoveAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to send a direct message.</p>
    fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> RusotoFuture<SendMessagesResponse, SendMessagesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.message_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendMessagesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendMessagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to send a message to a list of users.</p>
    fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> RusotoFuture<SendUsersMessagesResponse, SendUsersMessagesError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users-messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.send_users_message_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendUsersMessagesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendUsersMessagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an ADM channel.</p>
    fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> RusotoFuture<UpdateAdmChannelResponse, UpdateAdmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.adm_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAdmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAdmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update the APNs channel for an app.</p>
    fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> RusotoFuture<UpdateApnsChannelResponse, UpdateApnsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApnsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApnsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an APNS sandbox channel.</p>
    fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsSandboxChannelResponse, UpdateApnsSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApnsSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApnsSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Update an APNS VoIP channel</p>
    fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipChannelResponse, UpdateApnsVoipChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApnsVoipChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateApnsVoipChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Update an APNS VoIP sandbox channel</p>
    fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> RusotoFuture<UpdateApnsVoipSandboxChannelResponse, UpdateApnsVoipSandboxChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApnsVoipSandboxChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApnsVoipSandboxChannelError::from_response(response))
                }))
            }
        })
    }

    /// <p>Used to update the settings for an app.</p>
    fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> RusotoFuture<UpdateApplicationSettingsResponse, UpdateApplicationSettingsError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_application_settings_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApplicationSettingsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationSettingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Update a BAIDU GCM channel</p>
    fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> RusotoFuture<UpdateBaiduChannelResponse, UpdateBaiduChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.baidu_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateBaiduChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateBaiduChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update a campaign.</p>
    fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> RusotoFuture<UpdateCampaignResponse, UpdateCampaignError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCampaignResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateCampaignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an email channel.</p>
    fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> RusotoFuture<UpdateEmailChannelResponse, UpdateEmailChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateEmailChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEmailChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates or updates an endpoint.</p>
    fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> RusotoFuture<UpdateEndpointResponse, UpdateEndpointError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateEndpointResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Use to update a batch of endpoints.</p>
    fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> RusotoFuture<UpdateEndpointsBatchResponse, UpdateEndpointsBatchError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_batch_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateEndpointsBatchResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateEndpointsBatchError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Use to update the GCM channel for an app.</p>
    fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> RusotoFuture<UpdateGcmChannelResponse, UpdateGcmChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.gcm_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateGcmChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGcmChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used to update a segment.</p>
    fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> RusotoFuture<UpdateSegmentResponse, UpdateSegmentError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateSegmentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSegmentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an SMS channel.</p>
    fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> RusotoFuture<UpdateSmsChannelResponse, UpdateSmsChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateSmsChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateSmsChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an Voice channel</p>
    fn update_voice_channel(
        &self,
        input: UpdateVoiceChannelRequest,
    ) -> RusotoFuture<UpdateVoiceChannelResponse, UpdateVoiceChannelError> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.voice_channel_request).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateVoiceChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateVoiceChannelError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
