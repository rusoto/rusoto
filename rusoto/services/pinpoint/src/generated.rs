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
/// <p>Specifies the status and settings of the ADM (Amazon Device Messaging) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ADMChannelRequest {
    /// <p>The Client ID that you received from Amazon to send messages by using ADM.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The Client Secret that you received from Amazon to send messages by using ADM.</p>
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,
    /// <p>Specifies whether to enable the ADM channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Provides information about the status and settings of the ADM (Amazon Device Messaging) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ADMChannelResponse {
    /// <p>The unique identifier for the application that the ADM channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the ADM channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Specifies whether the ADM channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the ADM channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the ADM channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the ADM channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the ADM channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the ADM channel, this value is ADM.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the ADM channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the settings for a one-time message that's sent directly to an endpoint through the ADM (Amazon Device Messaging) channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ADMMessage {
    /// <p><p>The action to occur if the recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This action uses the deep-linking features of the Android platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The body of the notification message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>An arbitrary string that indicates that multiple messages are logically the same and that Amazon Device Messaging (ADM) can drop previously enqueued messages in favor of this message.</p>
    #[serde(rename = "ConsolidationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidation_key: Option<String>,
    /// <p>The JSON data payload to use for the push notification, if the notification is a silent push notification. This payload is added to the data.pinpoint.jsonBody object of the notification.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The amount of time, in seconds, that ADM should store the message if the recipient's device is offline. Amazon Pinpoint specifies this value in the expiresAfter parameter when it sends the notification message to ADM.</p>
    #[serde(rename = "ExpiresAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<String>,
    /// <p>The icon image name of the asset saved in your app.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL of the large icon image to display in the content view of the push notification.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL of an image to display in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The base64-encoded, MD5 checksum of the value specified by the Data property. ADM uses the MD5 value to verify the integrity of the data.</p>
    #[serde(rename = "MD5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for the notification message. If specified, this value overrides all other content for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Specifies whether the notification is a silent push notification, which is a push notification that doesn't display on a recipient's device. Silent push notifications can be used for cases such as updating an app's configuration or supporting phone home functionality.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL of the small icon image to display in the status bar and the content view of the push notification.</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>The sound to play when the recipient receives the push notification. You can use the default stream or specify the file name of a sound resource that's bundled in your app. On an Android platform, the sound file must reside in /res/raw/.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The default message variables to use in the notification message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The title to display above the notification message on the recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies the status and settings of the APNs (Apple Push Notification service) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct APNSChannelRequest {
    /// <p>The bundle identifier that's assigned to your iOS app. This identifier is used for APNs tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The APNs client certificate that you received from Apple, if you want Amazon Pinpoint to communicate with APNs by using an APNs certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method that you want Amazon Pinpoint to use when authenticating with APNs, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether to enable the APNs channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The private key for the APNs client certificate that you want Amazon Pinpoint to use to communicate with APNs.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The identifier that's assigned to your Apple developer account team. This identifier is used for APNs tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The authentication key to use for APNs tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The key identifier that's assigned to your APNs signing key, if you want Amazon Pinpoint to communicate with APNs by using APNs tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Provides information about the status and settings of the APNs (Apple Push Notification service) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct APNSChannelResponse {
    /// <p>The unique identifier for the application that the APNs channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the APNs channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method that Amazon Pinpoint uses to authenticate with APNs for this channel, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether the APNs channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Specifies whether the APNs channel is configured to communicate with APNs by using APNs tokens. To provide an authentication key for APNs tokens, set the TokenKey property of the channel.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>(Deprecated) An identifier for the APNs channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the APNs channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the APNs channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the APNs channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p><p>The type of messaging or notification platform for the channel. For the APNs channel, this value is APNS.</p></p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the APNs channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the settings for a one-time message that's sent directly to an endpoint through the APNs (Apple Push Notification service) channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct APNSMessage {
    /// <p>The type of push notification to send. Valid values are:</p> <ul><li><p>alert - For a standard notification that's displayed on recipients' devices and prompts a recipient to interact with the notification.</p></li> <li><p>background - For a silent notification that delivers content in the background and isn't displayed on recipients' devices.</p></li> <li><p>complication - For a notification that contains update information for an app’s complication timeline.</p></li> <li><p>fileprovider - For a notification that signals changes to a File Provider extension.</p></li> <li><p>mdm - For a notification that tells managed devices to contact the MDM server.</p></li> <li><p>voip - For a notification that provides information about an incoming VoIP call.</p></li></ul> <p>Amazon Pinpoint specifies this value in the apns-push-type request header when it sends the notification message to APNs. If you don't specify a value for this property, Amazon Pinpoint sets the value to alert or background automatically, based on the value that you specify for the SilentPush or RawContent property of the message.</p> <p>For more information about the apns-push-type request header, see <a href="https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/sending_notification_requests_to_apns">Sending Notification Requests to APNs</a> on the Apple Developer website.</p>
    #[serde(rename = "APNSPushType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_push_type: Option<String>,
    /// <p><p>The action to occur if the recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This setting uses the deep-linking features of the iOS platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The key that indicates whether and how to modify the badge of your app's icon when the recipient receives the push notification. If this key isn't included in the dictionary, the badge doesn't change. To remove the badge, set this value to 0.</p>
    #[serde(rename = "Badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<i64>,
    /// <p>The body of the notification message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The key that indicates the notification type for the push notification. This key is a value that's defined by the identifier property of one of your app's registered categories.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>An arbitrary identifier that, if assigned to multiple messages, APNs uses to coalesce the messages into a single push notification instead of delivering each message individually. This value can't exceed 64 bytes.</p> <p>Amazon Pinpoint specifies this value in the apns-collapse-id request header when it sends the notification message to APNs.</p>
    #[serde(rename = "CollapseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    /// <p>The JSON payload to use for a silent push notification. This payload is added to the data.pinpoint.jsonBody object of the notification.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The URL of an image or video to display in the push notification.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The authentication method that you want Amazon Pinpoint to use when authenticating with APNs, CERTIFICATE or TOKEN.</p>
    #[serde(rename = "PreferredAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_authentication_method: Option<String>,
    /// <p>para>5 - Low priority, the notification might be delayed, delivered as part of a group, or throttled.</p>/listitem> <li><p>10 - High priority, the notification is sent immediately. This is the default value. A high priority notification should trigger an alert, play a sound, or badge your app's icon on the recipient's device.</p></li>/para> <p>Amazon Pinpoint specifies this value in the apns-priority request header when it sends the notification message to APNs.</p> <p>The equivalent values for Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), are normal, for 5, and high, for 10. If you specify an FCM value for this property, Amazon Pinpoint accepts and converts the value to the corresponding APNs value.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p><p>The raw, JSON-formatted string to use as the payload for the notification message. If specified, this value overrides all other content for the message.</p> <note><p>If you specify the raw content of an APNs push notification, the message payload has to include the content-available key. The value of the content-available key has to be an integer, and can only be 0 or 1. If you&#39;re sending a standard notification, set the value of content-available to 0. If you&#39;re sending a silent (background) notification, set the value of content-available to 1. Additionally, silent notification payloads can&#39;t include the alert, badge, or sound keys. For more information, see <a href="https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification">Generating a Remote Notification</a> and <a href="https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/pushing_background_updates_to_your_app">Pushing Background Updates to Your App</a> on the Apple Developer website.</p></note></p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p><p>Specifies whether the notification is a silent push notification. A silent (or background) push notification isn&#39;t displayed on recipients&#39; devices. You can use silent push notifications to make small updates to your app, or to display messages in an in-app message center.</p> <p>Amazon Pinpoint uses this property to determine the correct value for the apns-push-type request header when it sends the notification message to APNs. If you specify a value of true for this property, Amazon Pinpoint sets the value for the apns-push-type header field to background.</p> <note><p>If you specify the raw content of an APNs push notification, the message payload has to include the content-available key. For silent (background) notifications, set the value of content-available to 1. Additionally, the message payload for a silent notification can&#39;t include the alert, badge, or sound keys. For more information, see <a href="https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification">Generating a Remote Notification</a> and <a href="https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/pushing_background_updates_to_your_app">Pushing Background Updates to Your App</a> on the Apple Developer website.</p> <p>Apple has indicated that they will throttle &quot;excessive&quot; background notifications based on current traffic volumes. To prevent your notifications being throttled, Apple recommends that you send no more than 3 silent push notifications to each recipient per hour.</p></note></p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The key for the sound to play when the recipient receives the push notification. The value for this key is the name of a sound file in your app's main bundle or the Library/Sounds folder in your app's data container. If the sound file can't be found or you specify default for the value, the system plays the default alert sound.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The default message variables to use in the notification message. You can override these default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The key that represents your app-specific identifier for grouping notifications. If you provide a Notification Content app extension, you can use this value to group your notifications together.</p>
    #[serde(rename = "ThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// <p>The amount of time, in seconds, that APNs should store and attempt to deliver the push notification, if the service is unable to deliver the notification the first time. If this value is 0, APNs treats the notification as if it expires immediately and the service doesn't store or try to deliver the notification again.</p> <p>Amazon Pinpoint specifies this value in the apns-expiration request header when it sends the notification message to APNs.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The title to display above the notification message on the recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies channel-specific content and settings for a message template that can be used in push notifications that are sent through the APNs (Apple Push Notification service) channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct APNSPushNotificationTemplate {
    /// <p><p>The action to occur if a recipient taps a push notification that&#39;s based on the message template. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This setting uses the deep-linking features of the iOS platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body to use in push notifications that are based on the message template.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The URL of an image or video to display in push notifications that are based on the message template.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for push notifications that are based on the message template. If specified, this value overrides all other content for the message template.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>The key for the sound to play when the recipient receives a push notification that's based on the message template. The value for this key is the name of a sound file in your app's main bundle or the Library/Sounds folder in your app's data container. If the sound file can't be found or you specify default for the value, the system plays the default alert sound.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The title to use in push notifications that are based on the message template. This title appears above the notification message on a recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the recipient's default mobile browser, if a recipient taps a push notification that's based on the message template and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies the status and settings of the APNs (Apple Push Notification service) sandbox channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct APNSSandboxChannelRequest {
    /// <p>The bundle identifier that's assigned to your iOS app. This identifier is used for APNs tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The APNs client certificate that you received from Apple, if you want Amazon Pinpoint to communicate with the APNs sandbox environment by using an APNs certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method that you want Amazon Pinpoint to use when authenticating with the APNs sandbox environment, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether to enable the APNs sandbox channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The private key for the APNs client certificate that you want Amazon Pinpoint to use to communicate with the APNs sandbox environment.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The identifier that's assigned to your Apple developer account team. This identifier is used for APNs tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The authentication key to use for APNs tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The key identifier that's assigned to your APNs signing key, if you want Amazon Pinpoint to communicate with the APNs sandbox environment by using APNs tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Provides information about the status and settings of the APNs (Apple Push Notification service) sandbox channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct APNSSandboxChannelResponse {
    /// <p>The unique identifier for the application that the APNs sandbox channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the APNs sandbox channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method that Amazon Pinpoint uses to authenticate with the APNs sandbox environment for this channel, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether the APNs sandbox channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Specifies whether the APNs sandbox channel is configured to communicate with APNs by using APNs tokens. To provide an authentication key for APNs tokens, set the TokenKey property of the channel.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>(Deprecated) An identifier for the APNs sandbox channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the APNs sandbox channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the APNs sandbox channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the APNs sandbox channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the APNs sandbox channel, this value is APNS_SANDBOX.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the APNs sandbox channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the status and settings of the APNs (Apple Push Notification service) VoIP channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct APNSVoipChannelRequest {
    /// <p>The bundle identifier that's assigned to your iOS app. This identifier is used for APNs tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The APNs client certificate that you received from Apple, if you want Amazon Pinpoint to communicate with APNs by using an APNs certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method that you want Amazon Pinpoint to use when authenticating with APNs, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether to enable the APNs VoIP channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The private key for the APNs client certificate that you want Amazon Pinpoint to use to communicate with APNs.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The identifier that's assigned to your Apple developer account team. This identifier is used for APNs tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The authentication key to use for APNs tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The key identifier that's assigned to your APNs signing key, if you want Amazon Pinpoint to communicate with APNs by using APNs tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Provides information about the status and settings of the APNs (Apple Push Notification service) VoIP channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct APNSVoipChannelResponse {
    /// <p>The unique identifier for the application that the APNs VoIP channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the APNs VoIP channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method that Amazon Pinpoint uses to authenticate with APNs for this channel, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether the APNs VoIP channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Specifies whether the APNs VoIP channel is configured to communicate with APNs by using APNs tokens. To provide an authentication key for APNs tokens, set the TokenKey property of the channel.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>(Deprecated) An identifier for the APNs VoIP channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the APNs VoIP channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the APNs VoIP channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the APNs VoIP channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the APNs VoIP channel, this value is APNS_VOIP.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the APNs VoIP channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the status and settings of the APNs (Apple Push Notification service) VoIP sandbox channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct APNSVoipSandboxChannelRequest {
    /// <p>The bundle identifier that's assigned to your iOS app. This identifier is used for APNs tokens.</p>
    #[serde(rename = "BundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The APNs client certificate that you received from Apple, if you want Amazon Pinpoint to communicate with the APNs sandbox environment by using an APNs certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// <p>The default authentication method that you want Amazon Pinpoint to use when authenticating with the APNs sandbox environment for this channel, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether the APNs VoIP sandbox channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The private key for the APNs client certificate that you want Amazon Pinpoint to use to communicate with the APNs sandbox environment.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The identifier that's assigned to your Apple developer account team. This identifier is used for APNs tokens.</p>
    #[serde(rename = "TeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// <p>The authentication key to use for APNs tokens.</p>
    #[serde(rename = "TokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    /// <p>The key identifier that's assigned to your APNs signing key, if you want Amazon Pinpoint to communicate with the APNs sandbox environment by using APNs tokens.</p>
    #[serde(rename = "TokenKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

/// <p>Provides information about the status and settings of the APNs (Apple Push Notification service) VoIP sandbox channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct APNSVoipSandboxChannelResponse {
    /// <p>The unique identifier for the application that the APNs VoIP sandbox channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the APNs VoIP sandbox channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The default authentication method that Amazon Pinpoint uses to authenticate with the APNs sandbox environment for this channel, key or certificate.</p>
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    /// <p>Specifies whether the APNs VoIP sandbox channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>Specifies whether the APNs VoIP sandbox channel is configured to communicate with APNs by using APNs tokens. To provide an authentication key for APNs tokens, set the TokenKey property of the channel.</p>
    #[serde(rename = "HasTokenKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    /// <p>(Deprecated) An identifier for the APNs VoIP sandbox channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the APNs VoIP sandbox channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the APNs VoIP sandbox channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the APNs VoIP sandbox channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the APNs VoIP sandbox channel, this value is APNS_VOIP_SANDBOX.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the APNs VoIP sandbox channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Provides information about the activities that were performed by a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivitiesResponse {
    /// <p>An array of responses, one for each activity that was performed by the campaign.</p>
    #[serde(rename = "Item")]
    pub item: Vec<ActivityResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Specifies the configuration and other settings for an activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    /// <p>The settings for a yes/no split activity. This type of activity sends participants down one of two paths in a journey, based on conditions that you specify.</p>
    #[serde(rename = "ConditionalSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_split: Option<ConditionalSplitActivity>,
    /// <p>The custom description of the activity.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The settings for an email activity. This type of activity sends an email message to participants.</p>
    #[serde(rename = "EMAIL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailMessageActivity>,
    /// <p>The settings for a holdout activity. This type of activity stops a journey for a specified percentage of participants.</p>
    #[serde(rename = "Holdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout: Option<HoldoutActivity>,
    /// <p>The settings for a multivariate split activity. This type of activity sends participants down one of as many as five paths (including a default <i>Else</i> path) in a journey, based on conditions that you specify.</p>
    #[serde(rename = "MultiCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_condition: Option<MultiConditionalSplitActivity>,
    /// <p>The settings for a random split activity. This type of activity randomly sends specified percentages of participants down one of as many as five paths in a journey, based on conditions that you specify.</p>
    #[serde(rename = "RandomSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_split: Option<RandomSplitActivity>,
    /// <p>The settings for a wait activity. This type of activity waits for a certain amount of time or until a specific date and time before moving participants to the next activity in a journey.</p>
    #[serde(rename = "Wait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<WaitActivity>,
}

/// <p>Provides information about an activity that was performed by a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivityResponse {
    /// <p>The unique identifier for the application that the campaign applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign that the activity applies to.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The actual time, in ISO 8601 format, when the activity was marked CANCELLED or COMPLETED.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The unique identifier for the activity.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Specifies whether the activity succeeded. Possible values are SUCCESS and FAIL.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The scheduled start time, in ISO 8601 format, for the activity.</p>
    #[serde(rename = "ScheduledStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<String>,
    /// <p>The actual start time, in ISO 8601 format, of the activity.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>The current status of the activity. Possible values are: PENDING, INITIALIZING, RUNNING, PAUSED, CANCELLED, and COMPLETED.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The total number of endpoints that the campaign successfully delivered messages to.</p>
    #[serde(rename = "SuccessfulEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_endpoint_count: Option<i64>,
    /// <p>The total number of time zones that were completed.</p>
    #[serde(rename = "TimezonesCompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_completed_count: Option<i64>,
    /// <p>The total number of unique time zones that are in the segment for the campaign.</p>
    #[serde(rename = "TimezonesTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_total_count: Option<i64>,
    /// <p>The total number of endpoints that the campaign attempted to deliver messages to.</p>
    #[serde(rename = "TotalEndpointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_endpoint_count: Option<i64>,
    /// <p>The unique identifier for the campaign treatment that the activity applies to. A treatment is a variation of a campaign that's used for A/B testing of a campaign.</p>
    #[serde(rename = "TreatmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_id: Option<String>,
}

/// <p>Specifies address-based configuration settings for a message that's sent directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddressConfiguration {
    /// <p>The message body to use instead of the default message body. This value overrides the default message body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>The channel to use when sending the message.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>An object that maps custom attributes to attributes for the address and is attached to the message. For a push notification, this payload is added to the data.pinpoint object. For an email or text message, this payload is added to email/SMS delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The raw, JSON-formatted string to use as the payload for the message. If specified, this value overrides all other values for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of the message variables to merge with the variables specified by properties of the DefaultMessage object. The variables specified in this map take precedence over all other variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The message title to use instead of the default message title. This value overrides the default message title.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Specifies channel-specific content and settings for a message template that can be used in push notifications that are sent through the ADM (Amazon Device Messaging), Baidu (Baidu Cloud Push), or GCM (Firebase Cloud Messaging, formerly Google Cloud Messaging) channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AndroidPushNotificationTemplate {
    /// <p><p>The action to occur if a recipient taps a push notification that&#39;s based on the message template. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This action uses the deep-linking features of the Android platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body to use in a push notification that's based on the message template.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The URL of the large icon image to display in the content view of a push notification that's based on the message template.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL of an image to display in a push notification that's based on the message template.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for a push notification that's based on the message template. If specified, this value overrides all other content for the message template.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>The URL of the small icon image to display in the status bar and the content view of a push notification that's based on the message template.</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>The sound to play when a recipient receives a push notification that's based on the message template. You can use the default stream or specify the file name of a sound resource that's bundled in your app. On an Android platform, the sound file must reside in /res/raw/.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The title to use in a push notification that's based on the message template. This title appears above the notification message on a recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in a recipient's default mobile browser, if a recipient taps a a push notification that's based on the message template and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Provides the results of a query that retrieved the data for a standard metric that applies to an application, and provides information about that query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationDateRangeKpiResponse {
    /// <p>The unique identifier for the application that the metric applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The last date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, that the data was retrieved for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. For a list of possible values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>An array of objects that contains the results of the query. Each object contains the value for the metric and metadata about that value.</p>
    #[serde(rename = "KpiResult")]
    pub kpi_result: BaseKpiResult,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null for the Application Metrics resource because the resource returns all results in a single page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The first date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

/// <p>Provides information about an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationResponse {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The display name of the application. This name is displayed as the <b>Project name</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the application. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about an application, including the default settings for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationSettingsResource {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The settings for the AWS Lambda function to use by default as a code hook for campaigns in the application.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>The date and time, in ISO 8601 format, when the application's settings were last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The default sending limits for campaigns in the application.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for campaigns and journeys in the application. Quiet time is a specific time range when messages aren't sent to endpoints, if all the following conditions are met:</p> <ul><li><p>The EndpointDemographic.Timezone property of the endpoint is set to a valid value.</p></li> <li><p>The current time in the endpoint's time zone is later than or equal to the time specified by the QuietTime.Start property for the application (or a campaign or journey that has custom quiet time settings).</p></li> <li><p>The current time in the endpoint's time zone is earlier than or equal to the time specified by the QuietTime.End property for the application (or a campaign or journey that has custom quiet time settings).</p></li></ul> <p>If any of the preceding conditions isn't met, the endpoint will receive messages from a campaign or journey, even if quiet time is enabled.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Provides information about all of your applications.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationsResponse {
    /// <p>An array of responses, one for each application that was returned.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ApplicationResponse>>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Specifies attribute-based criteria for including or excluding endpoints from a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeDimension {
    /// <p>The type of segment dimension to use. Valid values are: INCLUSIVE, endpoints that match the criteria are included in the segment; and, EXCLUSIVE, endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    /// <p>The criteria values to use for the segment dimension. Depending on the value of the AttributeType property, endpoints are included or excluded from the segment if their attribute values match the criteria values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Provides information about the type and the names of attributes that were removed from all the endpoints that are associated with an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttributesResource {
    /// <p>The unique identifier for the application.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p><p>The type of attribute or attributes that were removed from the endpoints. Valid values are:</p> <ul><li><p>endpoint-custom-attributes - Custom attributes that describe endpoints.</p></li> <li><p>endpoint-metric-attributes - Custom metrics that your app reports to Amazon Pinpoint for endpoints.</p></li> <li><p>endpoint-user-attributes - Custom attributes that describe users.</p></li></ul></p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    /// <p>An array that specifies the names of the attributes that were removed from the endpoints.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

/// <p>Specifies the status and settings of the Baidu (Baidu Cloud Push) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BaiduChannelRequest {
    /// <p>The API key that you received from the Baidu Cloud Push service to communicate with the service.</p>
    #[serde(rename = "ApiKey")]
    pub api_key: String,
    /// <p>Specifies whether to enable the Baidu channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The secret key that you received from the Baidu Cloud Push service to communicate with the service.</p>
    #[serde(rename = "SecretKey")]
    pub secret_key: String,
}

/// <p>Provides information about the status and settings of the Baidu (Baidu Cloud Push) channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BaiduChannelResponse {
    /// <p>The unique identifier for the application that the Baidu channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the Baidu channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The API key that you received from the Baidu Cloud Push service to communicate with the service.</p>
    #[serde(rename = "Credential")]
    pub credential: String,
    /// <p>Specifies whether the Baidu channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the Baidu channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the Baidu channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the Baidu channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the Baidu channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the Baidu channel, this value is BAIDU.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the Baidu channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the settings for a one-time message that's sent directly to an endpoint through the Baidu (Baidu Cloud Push) channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BaiduMessage {
    /// <p><p>The action to occur if the recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This action uses the deep-linking features of the Android platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The body of the notification message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The JSON data payload to use for the push notification, if the notification is a silent push notification. This payload is added to the data.pinpoint.jsonBody object of the notification.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your app.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL of the large icon image to display in the content view of the push notification.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL of an image to display in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for the notification message. If specified, this value overrides all other content for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Specifies whether the notification is a silent push notification, which is a push notification that doesn't display on a recipient's device. Silent push notifications can be used for cases such as updating an app's configuration or supporting phone home functionality.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL of the small icon image to display in the status bar and the content view of the push notification.</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>The sound to play when the recipient receives the push notification. You can use the default stream or specify the file name of a sound resource that's bundled in your app. On an Android platform, the sound file must reside in /res/raw/.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The default message variables to use in the notification message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The amount of time, in seconds, that the Baidu Cloud Push service should store the message if the recipient's device is offline. The default value and maximum supported time is 604,800 seconds (7 days).</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The title to display above the notification message on the recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Provides the results of a query that retrieved the data for a standard metric that applies to an application, campaign, or journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BaseKpiResult {
    /// <p>An array of objects that provides the results of a query that retrieved the data for a standard metric that applies to an application, campaign, or journey.</p>
    #[serde(rename = "Rows")]
    pub rows: Vec<ResultRow>,
}

/// <p>Provides the results of a query that retrieved the data for a standard metric that applies to a campaign, and provides information about that query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignDateRangeKpiResponse {
    /// <p>The unique identifier for the application that the metric applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign that the metric applies to.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The last date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, that the data was retrieved for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. For a list of possible values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>An array of objects that contains the results of the query. Each object contains the value for the metric and metadata about that value.</p>
    #[serde(rename = "KpiResult")]
    pub kpi_result: BaseKpiResult,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null for the Campaign Metrics resource because the resource returns all results in a single page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The first date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

/// <p>Specifies the content and "From" address for an email message that's sent to recipients of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignEmailMessage {
    /// <p>The body of the email for recipients whose email clients don't render HTML content.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The verified email address to send the email from. The default address is the FromAddress specified for the email channel for the application.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The body of the email, in HTML format, for recipients whose email clients render HTML content.</p>
    #[serde(rename = "HtmlBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    /// <p>The subject line, or title, of the email.</p>
    #[serde(rename = "Title")]
    pub title: String,
}

/// <p>Specifies the settings for events that cause a campaign to be sent.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignEventFilter {
    /// <p>The dimension settings of the event filter for the campaign.</p>
    #[serde(rename = "Dimensions")]
    pub dimensions: EventDimensions,
    /// <p>The type of event that causes the campaign to be sent. Valid values are: SYSTEM, sends the campaign when a system event occurs; and, ENDPOINT, sends the campaign when an endpoint event (<link  linkend="apps-application-id-events">Events</link> resource) occurs.</p>
    #[serde(rename = "FilterType")]
    pub filter_type: String,
}

/// <p>Specifies the AWS Lambda function to use as a code hook for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignHook {
    /// <p>The name or Amazon Resource Name (ARN) of the AWS Lambda function that Amazon Pinpoint invokes to send messages for a campaign.</p>
    #[serde(rename = "LambdaFunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<String>,
    /// <p>Specifies which Lambda mode to use when invoking the AWS Lambda function.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p><p>The web URL that Amazon Pinpoint calls to invoke the AWS Lambda function over HTTPS.</p></p>
    #[serde(rename = "WebUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// <p>Specifies limits on the messages that a campaign can send.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignLimits {
    /// <p>The maximum number of messages that a campaign can send to a single endpoint during a 24-hour period. The maximum value is 100.</p>
    #[serde(rename = "Daily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<i64>,
    /// <p>The maximum amount of time, in seconds, that a campaign can attempt to deliver a message after the scheduled start time for the campaign. The minimum value is 60 seconds.</p>
    #[serde(rename = "MaximumDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<i64>,
    /// <p>The maximum number of messages that a campaign can send each second. The minimum value is 50. The maximum value is 20,000.</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>The maximum number of messages that a campaign can send to a single endpoint during the course of the campaign. The maximum value is 100.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// <p>Provides information about the status, configuration, and other settings for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignResponse {
    /// <p>An array of responses, one for each treatment that you defined for the campaign, in addition to the default treatment.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<TreatmentResource>>,
    /// <p>The unique identifier for the application that the campaign applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The date, in ISO 8601 format, when the campaign was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The current status of the campaign's default treatment. This value exists only for campaigns that have more than one treatment, to support A/B testing.</p>
    #[serde(rename = "DefaultState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_state: Option<CampaignState>,
    /// <p>The custom description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of users (segment members) who shouldn't receive messages from the campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>The settings for the AWS Lambda function to use as a code hook for the campaign.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Specifies whether the campaign is paused. A paused campaign doesn't run unless you resume it by changing this value to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The date, in ISO 8601 format, when the campaign was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The messaging limits for the campaign.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings for the campaign.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schedule settings for the campaign.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The unique identifier for the segment that's associated with the campaign.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The version number of the segment that's associated with the campaign.</p>
    #[serde(rename = "SegmentVersion")]
    pub segment_version: i64,
    /// <p>The current status of the campaign.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>The message template that’s used for the campaign.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>The custom description of a variation of the campaign that's used for A/B testing.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign that's used for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    /// <p>The version number of the campaign.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the campaign. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the content and settings for an SMS message that's sent to recipients of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CampaignSmsMessage {
    /// <p>The body of the SMS message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The type of SMS message. Valid values are: TRANSACTIONAL, the message is critical or time-sensitive, such as a one-time password that supports a customer transaction; and, PROMOTIONAL, the message isn't critical or time-sensitive, such as a marketing message.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The sender ID to display on recipients' devices when they receive the SMS message.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

/// <p>Provides information about the status of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignState {
    /// <p>The current status of the campaign, or the current status of a treatment that belongs to an A/B test campaign. If a campaign uses A/B testing, the campaign has a status of COMPLETED only if all campaign treatments have a status of COMPLETED.</p>
    #[serde(rename = "CampaignStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_status: Option<String>,
}

/// <p>Provides information about the configuration and other settings for all the campaigns that are associated with an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignsResponse {
    /// <p>An array of responses, one for each campaign that's associated with the application.</p>
    #[serde(rename = "Item")]
    pub item: Vec<CampaignResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Provides information about the general settings and status of a channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChannelResponse {
    /// <p>The unique identifier for the application.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Specifies whether the channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The current version of the channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Provides information about the general settings and status of all channels for an application, including channels that aren't enabled for the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChannelsResponse {
    /// <p>A map that contains a multipart response for each channel. For each item in this object, the ChannelType is the key and the Channel is the value.</p>
    #[serde(rename = "Channels")]
    pub channels: ::std::collections::HashMap<String, ChannelResponse>,
}

/// <p>Specifies the conditions to evaluate for an activity in a journey, and how to evaluate those conditions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// <p>The conditions to evaluate for the activity.</p>
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SimpleCondition>>,
    /// <p>Specifies how to handle multiple conditions for the activity. For example, if you specify two conditions for an activity, whether both or only one of the conditions must be met for the activity to be performed.</p>
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

/// <p>Specifies the settings for a yes/no split activity in a journey. This type of activity sends participants down one of two paths in a journey, based on conditions that you specify.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalSplitActivity {
    /// <p>The conditions that define the paths for the activity, and the relationship between the conditions.</p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// <p>The amount of time to wait before determining whether the conditions are met, or the date and time when Amazon Pinpoint determines whether the conditions are met.</p>
    #[serde(rename = "EvaluationWaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_wait_time: Option<WaitTime>,
    /// <p>The unique identifier for the activity to perform if the conditions aren't met.</p>
    #[serde(rename = "FalseActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub false_activity: Option<String>,
    /// <p>The unique identifier for the activity to perform if the conditions are met.</p>
    #[serde(rename = "TrueActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_activity: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAppRequest {
    #[serde(rename = "CreateApplicationRequest")]
    pub create_application_request: CreateApplicationRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

/// <p>Specifies the display name of an application and the tags to associate with the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p>The display name of the application. This name is displayed as the <b>Project name</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the application. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCampaignRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEmailTemplateRequest {
    #[serde(rename = "EmailTemplateRequest")]
    pub email_template_request: EmailTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEmailTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    pub create_template_message_body: CreateTemplateMessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateExportJobRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ExportJobRequest")]
    pub export_job_request: ExportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateImportJobRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "ImportJobRequest")]
    pub import_job_request: ImportJobRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJourneyRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteJourneyRequest")]
    pub write_journey_request: WriteJourneyRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    pub journey_response: JourneyResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePushTemplateRequest {
    #[serde(rename = "PushNotificationTemplateRequest")]
    pub push_notification_template_request: PushNotificationTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePushTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    pub create_template_message_body: CreateTemplateMessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSegmentRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSmsTemplateRequest {
    #[serde(rename = "SMSTemplateRequest")]
    pub sms_template_request: SMSTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSmsTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    pub create_template_message_body: CreateTemplateMessageBody,
}

/// <p>Provides information about a request to create a message template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTemplateMessageBody {
    /// <p>The Amazon Resource Name (ARN) of the message template that was created.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The message that's returned from the API for the request to create the message template.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The unique identifier for the request to create the message template.</p>
    #[serde(rename = "RequestID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVoiceTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    #[serde(rename = "VoiceTemplateRequest")]
    pub voice_template_request: VoiceTemplateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVoiceTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    pub create_template_message_body: CreateTemplateMessageBody,
}

/// <p>Specifies the default message for all channels.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DefaultMessage {
    /// <p>The default body of the message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The default message variables to use in the message. You can override these default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Specifies the default settings and content for a push notification that's sent directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DefaultPushNotificationMessage {
    /// <p><p>The default action to occur if a recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This setting uses the deep-linking features of the iOS and Android platforms.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The default body of the notification message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The JSON data payload to use for the default push notification, if the notification is a silent push notification. This payload is added to the data.pinpoint.jsonBody object of the notification.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether the default notification is a silent push notification, which is a push notification that doesn't display on a recipient's device. Silent push notifications can be used for cases such as updating an app's configuration or delivering messages to an in-app notification center.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The default message variables to use in the notification message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The default title to display above the notification message on a recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The default URL to open in a recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies the default settings and content for a message template that can be used in messages that are sent through a push notification channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultPushNotificationTemplate {
    /// <p><p>The action to occur if a recipient taps a push notification that&#39;s based on the message template. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This setting uses the deep-linking features of the iOS and Android platforms.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The message body to use in push notifications that are based on the message template.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The sound to play when a recipient receives a push notification that's based on the message template. You can use the default stream or specify the file name of a sound resource that's bundled in your app. On an Android platform, the sound file must reside in /res/raw/.</p> <p>For an iOS platform, this value is the key for the name of a sound file in your app's main bundle or the Library/Sounds folder in your app's data container. If the sound file can't be found or you specify default for the value, the system plays the default alert sound.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The title to use in push notifications that are based on the message template. This title appears above the notification message on a recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in a recipient's default mobile browser, if a recipient taps a push notification that's based on the message template and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAdmChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApnsChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApnsSandboxChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApnsVoipChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApnsVoipSandboxChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBaiduChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCampaignRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventStreamRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGcmChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJourneyRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    pub journey_response: JourneyResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePushTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePushTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSegmentRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSmsChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSmsTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSmsTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserEndpointsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVoiceTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVoiceTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

/// <p>Specifies the settings and content for the default message and any default messages that you tailored for specific channels.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DirectMessageConfiguration {
    /// <p>The default push notification message for the ADM (Amazon Device Messaging) channel. This message overrides the default push notification message (DefaultPushNotificationMessage).</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<ADMMessage>,
    /// <p>The default push notification message for the APNs (Apple Push Notification service) channel. This message overrides the default push notification message (DefaultPushNotificationMessage).</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<APNSMessage>,
    /// <p>The default push notification message for the Baidu (Baidu Cloud Push) channel. This message overrides the default push notification message (DefaultPushNotificationMessage).</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<BaiduMessage>,
    /// <p>The default message for all channels.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<DefaultMessage>,
    /// <p>The default push notification message for all push notification channels.</p>
    #[serde(rename = "DefaultPushNotificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_push_notification_message: Option<DefaultPushNotificationMessage>,
    /// <p>The default message for the email channel. This message overrides the default message (DefaultMessage).</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<EmailMessage>,
    /// <p>The default push notification message for the GCM channel, which is used to send notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. This message overrides the default push notification message (DefaultPushNotificationMessage).</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<GCMMessage>,
    /// <p>The default message for the SMS channel. This message overrides the default message (DefaultMessage).</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<SMSMessage>,
    /// <p>The default message for the voice channel. This message overrides the default message (DefaultMessage).</p>
    #[serde(rename = "VoiceMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_message: Option<VoiceMessage>,
}

/// <p>Specifies the status and settings of the email channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EmailChannelRequest {
    /// <p>The configuration set that you want to apply to email that you send through the channel by using the <a href="emailAPIreference.html">Amazon Pinpoint Email API</a>.</p>
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    /// <p>Specifies whether to enable the email channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p><p>The verified email address that you want to send email from when you send email through the channel.</p></p>
    #[serde(rename = "FromAddress")]
    pub from_address: String,
    /// <p><p>The Amazon Resource Name (ARN) of the identity, verified with Amazon Simple Email Service (Amazon SES), that you want to use when you send email through the channel.</p></p>
    #[serde(rename = "Identity")]
    pub identity: String,
    /// <p><p>The ARN of the AWS Identity and Access Management (IAM) role that you want Amazon Pinpoint to use when it submits email-related event data for the channel.</p></p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Provides information about the status and settings of the email channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EmailChannelResponse {
    /// <p>The unique identifier for the application that the email channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The configuration set that's applied to email that's sent through the channel by using the <a href="emailAPIreference.html">Amazon Pinpoint Email API</a>.</p>
    #[serde(rename = "ConfigurationSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the email channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Specifies whether the email channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The verified email address that you send email from when you send email through the channel.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the email channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The Amazon Resource Name (ARN) of the identity, verified with Amazon Simple Email Service (Amazon SES), that you use when you send email through the channel.</p></p>
    #[serde(rename = "Identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// <p>Specifies whether the email channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the email channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the email channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The maximum number of emails that you can send through the channel each second.</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
    /// <p>The type of messaging or notification platform for the channel. For the email channel, this value is EMAIL.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p><p>The ARN of the AWS Identity and Access Management (IAM) role that Amazon Pinpoint uses to submit email-related event data for the channel.</p></p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The current version of the email channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the default settings and content for a one-time email message that's sent directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EmailMessage {
    /// <p>The body of the email message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The email address to forward bounces and complaints to, if feedback forwarding is enabled.</p>
    #[serde(rename = "FeedbackForwardingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_address: Option<String>,
    /// <p>The verified email address to send the email message from. The default value is the FromAddress specified for the email channel.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The email message, represented as a raw MIME message.</p>
    #[serde(rename = "RawEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_email: Option<RawEmail>,
    /// <p>The reply-to email address(es) for the email message. If a recipient replies to the email, each reply-to address receives the reply.</p>
    #[serde(rename = "ReplyToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    /// <p>The email message, composed of a subject, a text part, and an HTML part.</p>
    #[serde(rename = "SimpleEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_email: Option<SimpleEmail>,
    /// <p>The default message variables to use in the email message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Specifies the settings for an email activity in a journey. This type of activity sends an email message to participants.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailMessageActivity {
    /// <p>The "From" address to use for the message.</p>
    #[serde(rename = "MessageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_config: Option<JourneyEmailMessage>,
    /// <p>The unique identifier for the next activity to perform, after the message is sent.</p>
    #[serde(rename = "NextActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    /// <p>The name of the email template to use for the message.</p>
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// <p>The unique identifier for the version of the email template to use for the message. If specified, this value must match the identifier for an existing template version. To retrieve a list of versions and version identifiers for a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If you don't specify a value for this property, Amazon Pinpoint uses the <i>active</i> version of the template. The <i>active</i> version is typically the version of a template that's been most recently reviewed and approved for use, depending on your workflow. It isn't necessarily the latest version of a template.</p>
    #[serde(rename = "TemplateVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

/// <p>Specifies the content and settings for a message template that can be used in messages that are sent through the email channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EmailTemplateRequest {
    /// <p>A JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The message body, in HTML format, to use in email messages that are based on the message template. We recommend using HTML format for email clients that render HTML content. You can include links, formatted text, and more in an HTML message.</p>
    #[serde(rename = "HtmlPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    /// <p>The subject line, or title, to use in email messages that are based on the message template.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>A custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The message body, in plain text format, to use in email messages that are based on the message template. We recommend using plain text format for email clients that don't render HTML content and clients that are connected to high-latency networks, such as mobile devices.</p>
    #[serde(rename = "TextPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about the content and settings for a message template that can be used in messages that are sent through the email channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EmailTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the message template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The JSON object that specifies the default values that are used for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The message body, in HTML format, that's used in email messages that are based on the message template.</p>
    #[serde(rename = "HtmlPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The subject line, or title, that's used in email messages that are based on the message template.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>The custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. For an email template, this value is EMAIL.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The message body, in plain text format, that's used in email messages that are based on the message template.</p>
    #[serde(rename = "TextPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
    /// <p>The unique identifier, as an integer, for the active version of the message template, or the version of the template that you specified by using the version parameter in your request.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies an endpoint to create or update and the settings and attributes to set or change for the endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointBatchItem {
    /// <p>The destination address for messages or push notifications that you send to the endpoint. The address varies by channel. For a push-notification channel, use the token provided by the push notification service, such as an Apple Push Notification service (APNs) device token or a Firebase Cloud Messaging (FCM) registration token. For the SMS channel, use a phone number in E.164 format, such as +12065550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>One or more custom attributes that describe the endpoint by associating a name with an array of values. For example, the value of a custom attribute named Interests might be: ["science", "music", "travel"]. You can use these attributes as filter criteria when you create segments.</p> <p>When you define the name of a custom attribute, avoid using the following characters: number sign (#), colon (:), question mark (?), backslash (\), and slash (/). The Amazon Pinpoint console can't display attribute names that contain these characters. This limitation doesn't apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel to use when sending messages or push notifications to the endpoint.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The demographic information for the endpoint, such as the time zone and platform.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time, in ISO 8601 format, when the endpoint was created or updated.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Specifies whether to send messages or push notifications to the endpoint. Valid values are: ACTIVE, messages are sent to the endpoint; and, INACTIVE, messages aren’t sent to the endpoint.</p> <p>Amazon Pinpoint automatically sets this value to ACTIVE when you create an endpoint or update an existing endpoint. Amazon Pinpoint automatically sets this value to INACTIVE if you update another endpoint that has the same address specified by the Address property.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique identifier for the endpoint in the context of the batch.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The geographic information for the endpoint.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>One or more custom metrics that your app reports to Amazon Pinpoint for the endpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Specifies whether the user who's associated with the endpoint has opted out of receiving messages and push notifications from you. Possible values are: ALL, the user has opted out and doesn't want to receive any messages or push notifications; and, NONE, the user hasn't opted out and wants to receive all messages and push notifications.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique identifier for the request to create or update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>One or more custom user attributes that your app reports to Amazon Pinpoint for the user who's associated with the endpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Specifies a batch of endpoints to create or update and the settings and attributes to set or change for each endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointBatchRequest {
    /// <p>An array that defines the endpoints to create or update and, for each endpoint, the property values to set or change. An array can contain a maximum of 100 items.</p>
    #[serde(rename = "Item")]
    pub item: Vec<EndpointBatchItem>,
}

/// <p>Specifies demographic information about an endpoint, such as the applicable time zone and platform.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDemographic {
    /// <p>The version of the app that's associated with the endpoint.</p>
    #[serde(rename = "AppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// <p>The locale of the endpoint, in the following format: the ISO 639-1 alpha-2 code, followed by an underscore (_), followed by an ISO 3166-1 alpha-2 value.</p>
    #[serde(rename = "Locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The manufacturer of the endpoint device, such as apple or samsung.</p>
    #[serde(rename = "Make")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// <p>The model name or number of the endpoint device, such as iPhone or SM-G900F.</p>
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The model version of the endpoint device.</p>
    #[serde(rename = "ModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>The platform of the endpoint device, such as ios.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The platform version of the endpoint device.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The time zone of the endpoint, specified as a tz database name value, such as America/Los_Angeles.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Provides the status code and message that result from processing data for an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointItemResponse {
    /// <p>The custom message that's returned in the response as a result of processing the endpoint data.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code that's returned in the response as a result of processing the endpoint data.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

/// <p>Specifies geographic information about an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointLocation {
    /// <p>The name of the city where the endpoint is located.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region where the endpoint is located. For example, US for the United States.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The latitude coordinate of the endpoint location, rounded to one decimal place.</p>
    #[serde(rename = "Latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// <p>The longitude coordinate of the endpoint location, rounded to one decimal place.</p>
    #[serde(rename = "Longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// <p>The postal or ZIP code for the area where the endpoint is located.</p>
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// <p>The name of the region where the endpoint is located. For locations in the United States, this value is the name of a state.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>Provides information about the delivery status and results of sending a message directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointMessageResult {
    /// <p>The endpoint address that the message was delivered to.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p><p>The delivery status of the message. Possible values are:</p> <ul> <li><p>DUPLICATE - The endpoint address is a duplicate of another endpoint address. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li> <li><p>OPT<em>OUT - The user who&#39;s associated with the endpoint has opted out of receiving messages from you. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li> <li><p>PERMANENT</em>FAILURE - An error occurred when delivering the message to the endpoint. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li>    <li><p>SUCCESSFUL - The message was successfully delivered to the endpoint.</p></li> <li><p>TEMPORARY<em>FAILURE - A temporary error occurred. Amazon Pinpoint will attempt to deliver the message again later.</p></li> <li><p>THROTTLED - Amazon Pinpoint throttled the operation to send the message to the endpoint.</p></li> <li><p>TIMEOUT - The message couldn&#39;t be sent within the timeout period.</p></li> <li><p>UNKNOWN</em>FAILURE - An unknown error occurred.</p></li></ul></p>
    #[serde(rename = "DeliveryStatus")]
    pub delivery_status: String,
    /// <p>The unique identifier for the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>The downstream service status code for delivering the message.</p>
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
    /// <p>The status message for delivering the message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>For push notifications that are sent through the GCM channel, specifies whether the endpoint's device registration token was updated as part of delivering the message.</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>Specifies the channel type and other settings for an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointRequest {
    /// <p>The destination address for messages or push notifications that you send to the endpoint. The address varies by channel. For a push-notification channel, use the token provided by the push notification service, such as an Apple Push Notification service (APNs) device token or a Firebase Cloud Messaging (FCM) registration token. For the SMS channel, use a phone number in E.164 format, such as +12065550100. For the email channel, use an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>One or more custom attributes that describe the endpoint by associating a name with an array of values. For example, the value of a custom attribute named Interests might be: ["science", "music", "travel"]. You can use these attributes as filter criteria when you create segments.</p> <p>When you define the name of a custom attribute, avoid using the following characters: number sign (#), colon (:), question mark (?), backslash (\), and slash (/). The Amazon Pinpoint console can't display attribute names that contain these characters. This limitation doesn't apply to attribute values.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel to use when sending messages or push notifications to the endpoint.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The demographic information for the endpoint, such as the time zone and platform.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time, in ISO 8601 format, when the endpoint is updated.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Specifies whether to send messages or push notifications to the endpoint. Valid values are: ACTIVE, messages are sent to the endpoint; and, INACTIVE, messages aren’t sent to the endpoint.</p> <p>Amazon Pinpoint automatically sets this value to ACTIVE when you create an endpoint or update an existing endpoint. Amazon Pinpoint automatically sets this value to INACTIVE if you update another endpoint that has the same address specified by the Address property.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The geographic information for the endpoint.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>One or more custom metrics that your app reports to Amazon Pinpoint for the endpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Specifies whether the user who's associated with the endpoint has opted out of receiving messages and push notifications from you. Possible values are: ALL, the user has opted out and doesn't want to receive any messages or push notifications; and, NONE, the user hasn't opted out and wants to receive all messages and push notifications.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique identifier for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>One or more custom user attributes that describe the user who's associated with the endpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Provides information about the channel type and other settings for an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointResponse {
    /// <p>The destination address for messages or push notifications that you send to the endpoint. The address varies by channel. For example, the address for a push-notification channel is typically the token provided by a push notification service, such as an Apple Push Notification service (APNs) device token or a Firebase Cloud Messaging (FCM) registration token. The address for the SMS channel is a phone number in E.164 format, such as +12065550100. The address for the email channel is an email address.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>The unique identifier for the application that's associated with the endpoint.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>One or more custom attributes that describe the endpoint by associating a name with an array of values. For example, the value of a custom attribute named Interests might be: ["science", "music", "travel"]. You can use these attributes as filter criteria when you create segments.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel that's used when sending messages or push notifications to the endpoint.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>A number from 0-99 that represents the cohort that the endpoint is assigned to. Endpoints are grouped into cohorts randomly, and each cohort contains approximately 1 percent of the endpoints for an application. Amazon Pinpoint assigns cohorts to the holdout or treatment allocations for campaigns.</p>
    #[serde(rename = "CohortId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_id: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the endpoint was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The demographic information for the endpoint, such as the time zone and platform.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time, in ISO 8601 format, when the endpoint was last updated.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Specifies whether messages or push notifications are sent to the endpoint. Possible values are: ACTIVE, messages are sent to the endpoint; and, INACTIVE, messages aren’t sent to the endpoint.</p> <p>Amazon Pinpoint automatically sets this value to ACTIVE when you create an endpoint or update an existing endpoint. Amazon Pinpoint automatically sets this value to INACTIVE if you update another endpoint that has the same address specified by the Address property.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The unique identifier that you assigned to the endpoint. The identifier should be a globally unique identifier (GUID) to ensure that it doesn't conflict with other endpoint identifiers that are associated with the application.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The geographic information for the endpoint.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>One or more custom metrics that your app reports to Amazon Pinpoint for the endpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Specifies whether the user who's associated with the endpoint has opted out of receiving messages and push notifications from you. Possible values are: ALL, the user has opted out and doesn't want to receive any messages or push notifications; and, NONE, the user hasn't opted out and wants to receive all messages and push notifications.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>The unique identifier for the most recent request to update the endpoint.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>One or more custom user attributes that your app reports to Amazon Pinpoint for the user who's associated with the endpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Specifies the content, including message variables and attributes, to use in a message that's sent directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointSendConfiguration {
    /// <p>The body of the message. If specified, this value overrides the default message body.</p>
    #[serde(rename = "BodyOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    /// <p>A map of custom attributes to attach to the message for the address. For a push notification, this payload is added to the data.pinpoint object. For an email or text message, this payload is added to email/SMS delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The raw, JSON-formatted string to use as the payload for the message. If specified, this value overrides all other values for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>A map of the message variables to merge with the variables specified for the default message (DefaultMessage.Substitutions). The variables specified in this map take precedence over all other variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The title or subject line of the message. If specified, this value overrides the default message title or subject line.</p>
    #[serde(rename = "TitleOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

/// <p>Specifies data for one or more attributes that describe the user who's associated with an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointUser {
    /// <p>One or more custom attributes that describe the user by associating a name with an array of values. For example, the value of an attribute named Interests might be: ["science", "music", "travel"]. You can use these attributes as filter criteria when you create segments.</p> <p>When you define the name of a custom attribute, avoid using the following characters: number sign (#), colon (:), question mark (?), backslash (\), and slash (/). The Amazon Pinpoint console can't display attribute names that contain these characters. This limitation doesn't apply to attribute values.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The unique identifier for the user.</p>
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// <p>Provides information about all the endpoints that are associated with a user ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointsResponse {
    /// <p>An array of responses, one for each endpoint that's associated with the user ID.</p>
    #[serde(rename = "Item")]
    pub item: Vec<EndpointResponse>,
}

/// <p>Specifies information about an event that reports data to Amazon Pinpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Event {
    /// <p>The package name of the app that's recording the event.</p>
    #[serde(rename = "AppPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_package_name: Option<String>,
    /// <p>The title of the app that's recording the event.</p>
    #[serde(rename = "AppTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    /// <p>The version number of the app that's recording the event.</p>
    #[serde(rename = "AppVersionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version_code: Option<String>,
    /// <p>One or more custom attributes that are associated with the event.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the SDK that's running on the client device.</p>
    #[serde(rename = "ClientSdkVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_sdk_version: Option<String>,
    /// <p>The name of the event.</p>
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// <p>One or more custom metrics that are associated with the event.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>The name of the SDK that's being used to record the event.</p>
    #[serde(rename = "SdkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_name: Option<String>,
    /// <p>Information about the session in which the event occurred.</p>
    #[serde(rename = "Session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    /// <p>The date and time, in ISO 8601 format, when the event occurred.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

/// <p>Specifies the conditions to evaluate for an event that applies to an activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventCondition {
    /// <p>The dimensions for the event filter to use for the activity.</p>
    #[serde(rename = "Dimensions")]
    pub dimensions: EventDimensions,
    /// <p>The message identifier (message_id) for the message to use when determining whether message events meet the condition.</p>
    #[serde(rename = "MessageActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_activity: Option<String>,
}

/// <p>Specifies the dimensions for an event filter that determines when a campaign is sent or a journey activity is performed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDimensions {
    /// <p>One or more custom attributes that your application reports to Amazon Pinpoint. You can use these attributes as selection criteria when you create an event filter.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    /// <p>The name of the event that causes the campaign to be sent or the journey activity to be performed. This can be a standard type of event that Amazon Pinpoint generates, such as _email.delivered, or a custom event that's specific to your application.</p>
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<SetDimension>,
    /// <p>One or more custom metrics that your application reports to Amazon Pinpoint. You can use these metrics as selection criteria when you create an event filter.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricDimension>>,
}

/// <p>Provides the status code and message that result from processing an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventItemResponse {
    /// <p>A custom message that's returned in the response as a result of processing the event.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code that's returned in the response as a result of processing the event. Possible values are: 202, for events that were accepted; and, 400, for events that weren't valid.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

/// <p>Specifies settings for publishing event data to an Amazon Kinesis data stream or an Amazon Kinesis Data Firehose delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventStream {
    /// <p>The unique identifier for the application to publish event data for.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis data stream or Amazon Kinesis Data Firehose delivery stream to publish event data to.</p> <p>For a Kinesis data stream, the ARN format is: arn:aws:kinesis:<replaceable>region</replaceable>:<replaceable>account-id</replaceable>:stream/<replaceable>stream_name</replaceable>
    /// </p> <p>For a Kinesis Data Firehose delivery stream, the ARN format is: arn:aws:firehose:<replaceable>region</replaceable>:<replaceable>account-id</replaceable>:deliverystream/<replaceable>stream_name</replaceable>
    /// </p>
    #[serde(rename = "DestinationStreamArn")]
    pub destination_stream_arn: String,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to an external ID key in an IAM trust policy. Amazon Pinpoint previously used this value to assume an IAM role when publishing event data, but we removed this requirement. We don't recommend use of external IDs for IAM roles that are assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The date, in ISO 8601 format, when the event stream was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The IAM user who last modified the event stream.</p>
    #[serde(rename = "LastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    /// <p>The AWS Identity and Access Management (IAM) role that authorizes Amazon Pinpoint to publish event data to the stream in your AWS account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>Specifies a batch of endpoints and events to process.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventsBatch {
    /// <p>A set of properties and attributes that are associated with the endpoint.</p>
    #[serde(rename = "Endpoint")]
    pub endpoint: PublicEndpoint,
    /// <p>A set of properties that are associated with the event.</p>
    #[serde(rename = "Events")]
    pub events: ::std::collections::HashMap<String, Event>,
}

/// <p>Specifies a batch of events to process.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventsRequest {
    /// <p>The batch of events to process. For each item in a batch, the endpoint ID acts as a key that has an EventsBatch object as its value.</p>
    #[serde(rename = "BatchItem")]
    pub batch_item: ::std::collections::HashMap<String, EventsBatch>,
}

/// <p>Provides information about endpoints and the events that they're associated with.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventsResponse {
    /// <p>A map that contains a multipart response for each endpoint. For each item in this object, the endpoint ID is the key and the item response is the value. If no item response exists, the value can also be one of the following: 202, the request was processed successfully; or 400, the payload wasn't valid or required fields were missing.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<::std::collections::HashMap<String, ItemResponse>>,
}

/// <p>Specifies the settings for a job that exports endpoint definitions to an Amazon Simple Storage Service (Amazon S3) bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that authorizes Amazon Pinpoint to access the Amazon S3 location where you want to export endpoint definitions to.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The URL of the location in an Amazon Simple Storage Service (Amazon S3) bucket where you want to export endpoint definitions to. This location is typically a folder that contains multiple files. The URL should be in the following format: s3://<replaceable>bucket-name</replaceable>/<replaceable>folder-name</replaceable>/.</p>
    #[serde(rename = "S3UrlPrefix")]
    pub s3_url_prefix: String,
    /// <p>The identifier for the segment to export endpoint definitions from. If you don't specify this value, Amazon Pinpoint exports definitions for all the endpoints that are associated with the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to export endpoint definitions from, if specified.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Provides information about the resource settings for a job that exports endpoint definitions to a file. The file can be added directly to an Amazon Simple Storage Service (Amazon S3) bucket by using the Amazon Pinpoint API or downloaded directly to a computer by using the Amazon Pinpoint console.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportJobResource {
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that authorized Amazon Pinpoint to access the Amazon S3 location where the endpoint definitions were exported to.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The URL of the location in an Amazon Simple Storage Service (Amazon S3) bucket where the endpoint definitions were exported to. This location is typically a folder that contains multiple files. The URL should be in the following format: s3://<replaceable>bucket-name</replaceable>/<replaceable>folder-name</replaceable>/.</p>
    #[serde(rename = "S3UrlPrefix")]
    pub s3_url_prefix: String,
    /// <p>The identifier for the segment that the endpoint definitions were exported from. If this value isn't present, Amazon Pinpoint exported definitions for all the endpoints that are associated with the application.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment that the endpoint definitions were exported from.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
}

/// <p>Provides information about the status and settings of a job that exports endpoint definitions to a file. The file can be added directly to an Amazon Simple Storage Service (Amazon S3) bucket by using the Amazon Pinpoint API or downloaded directly to a computer by using the Amazon Pinpoint console.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportJobResponse {
    /// <p>The unique identifier for the application that's associated with the export job.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of pieces that were processed successfully (completed) by the export job, as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date, in ISO 8601 format, when the export job was completed.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date, in ISO 8601 format, when the export job was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The resource settings that apply to the export job.</p>
    #[serde(rename = "Definition")]
    pub definition: ExportJobResource,
    /// <p>The number of pieces that weren't processed successfully (failed) by the export job, as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>An array of entries, one for each of the first 100 entries that weren't processed successfully (failed) by the export job, if any.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique identifier for the export job.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The status of the export job. The job status is FAILED if Amazon Pinpoint wasn't able to process one or more pieces in the job.</p>
    #[serde(rename = "JobStatus")]
    pub job_status: String,
    /// <p>The total number of endpoint definitions that weren't processed successfully (failed) by the export job, typically because an error, such as a syntax error, occurred.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be processed to complete the export job. Each piece consists of an approximately equal portion of the endpoint definitions that are part of the export job.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The total number of endpoint definitions that were processed by the export job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. This value is EXPORT for export jobs.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Provides information about all the export jobs that are associated with an application or segment. An export job is a job that exports endpoint definitions to a file.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportJobsResponse {
    /// <p>An array of responses, one for each export job that's associated with the application (Export Jobs resource) or segment (Segment Export Jobs resource).</p>
    #[serde(rename = "Item")]
    pub item: Vec<ExportJobResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Specifies the status and settings of the GCM channel for an application. This channel enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GCMChannelRequest {
    /// <p>The Web API Key, also referred to as an <i>API_KEY</i> or <i>server key</i>, that you received from Google to communicate with Google services.</p>
    #[serde(rename = "ApiKey")]
    pub api_key: String,
    /// <p>Specifies whether to enable the GCM channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Provides information about the status and settings of the GCM channel for an application. The GCM channel enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GCMChannelResponse {
    /// <p>The unique identifier for the application that the GCM channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time when the GCM channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The Web API Key, also referred to as an <i>API_KEY</i> or <i>server key</i>, that you received from Google to communicate with Google services.</p>
    #[serde(rename = "Credential")]
    pub credential: String,
    /// <p>Specifies whether the GCM channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the GCM channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the GCM channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the GCM channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time when the GCM channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the GCM channel, this value is GCM.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the GCM channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the settings for a one-time message that's sent directly to an endpoint through the GCM channel. The GCM channel enables Amazon Pinpoint to send messages to the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GCMMessage {
    /// <p><p>The action to occur if the recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This action uses the deep-linking features of the Android platform.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The body of the notification message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>An arbitrary string that identifies a group of messages that can be collapsed to ensure that only the last message is sent when delivery can resume. This helps avoid sending too many instances of the same messages when the recipient's device comes online again or becomes active.</p> <p>Amazon Pinpoint specifies this value in the Firebase Cloud Messaging (FCM) collapse_key parameter when it sends the notification message to FCM.</p>
    #[serde(rename = "CollapseKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
    /// <p>The JSON data payload to use for the push notification, if the notification is a silent push notification. This payload is added to the data.pinpoint.jsonBody object of the notification.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    /// <p>The icon image name of the asset saved in your app.</p>
    #[serde(rename = "IconReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    /// <p>The URL of the large icon image to display in the content view of the push notification.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL of an image to display in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>para>normal - The notification might be delayed. Delivery is optimized for battery usage on the recipient's device. Use this value unless immediate delivery is required.</p>/listitem> <li><p>high - The notification is sent immediately and might wake a sleeping device.</p></li>/para> <p>Amazon Pinpoint specifies this value in the FCM priority parameter when it sends the notification message to FCM.</p> <p>The equivalent values for Apple Push Notification service (APNs) are 5, for normal, and 10, for high. If you specify an APNs value for this property, Amazon Pinpoint accepts and converts the value to the corresponding FCM value.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for the notification message. If specified, this value overrides all other content for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>The package name of the application where registration tokens must match in order for the recipient to receive the message.</p>
    #[serde(rename = "RestrictedPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,
    /// <p>Specifies whether the notification is a silent push notification, which is a push notification that doesn't display on a recipient's device. Silent push notifications can be used for cases such as updating an app's configuration or supporting phone home functionality.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The URL of the small icon image to display in the status bar and the content view of the push notification.</p>
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    /// <p>The sound to play when the recipient receives the push notification. You can use the default stream or specify the file name of a sound resource that's bundled in your app. On an Android platform, the sound file must reside in /res/raw/.</p>
    #[serde(rename = "Sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    /// <p>The default message variables to use in the notification message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The amount of time, in seconds, that FCM should store and attempt to deliver the push notification, if the service is unable to deliver the notification the first time. If you don't specify this value, FCM defaults to the maximum value, which is 2,419,200 seconds (28 days).</p> <p>Amazon Pinpoint specifies this value in the FCM time_to_live parameter when it sends the notification message to FCM.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The title to display above the notification message on the recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in the recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Specifies the GPS coordinates of a location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSCoordinates {
    /// <p>The latitude coordinate of the location.</p>
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    /// <p>The longitude coordinate of the location.</p>
    #[serde(rename = "Longitude")]
    pub longitude: f64,
}

/// <p>Specifies GPS-based criteria for including or excluding endpoints from a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSPointDimension {
    /// <p>The GPS coordinates to measure distance from.</p>
    #[serde(rename = "Coordinates")]
    pub coordinates: GPSCoordinates,
    /// <p>The range, in kilometers, from the GPS coordinates.</p>
    #[serde(rename = "RangeInKilometers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_kilometers: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAdmChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApnsChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApnsSandboxChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApnsVoipChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApnsVoipSandboxChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppResponse {
    #[serde(rename = "ApplicationResponse")]
    pub application_response: ApplicationResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationDateRangeKpiRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The last date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format and use Coordinated Universal Time (UTC), for example: 2019-07-26T20:00:00Z for 8:00 PM UTC July 26, 2019.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, to retrieve data for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. Examples are email-open-rate and successful-delivery-rate. For a list of valid values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The first date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format and use Coordinated Universal Time (UTC), for example: 2019-07-19T20:00:00Z for 8:00 PM UTC July 19, 2019. This value should also be fewer than 90 days from the current day.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApplicationDateRangeKpiResponse {
    #[serde(rename = "ApplicationDateRangeKpiResponse")]
    pub application_date_range_kpi_response: ApplicationDateRangeKpiResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetApplicationSettingsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppsRequest {
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppsResponse {
    #[serde(rename = "ApplicationsResponse")]
    pub applications_response: ApplicationsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBaiduChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignActivitiesRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignActivitiesResponse {
    #[serde(rename = "ActivitiesResponse")]
    pub activities_response: ActivitiesResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignDateRangeKpiRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The last date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format and use Coordinated Universal Time (UTC), for example: 2019-07-26T20:00:00Z for 8:00 PM UTC July 26, 2019.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, to retrieve data for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. Examples are email-open-rate and successful-delivery-rate. For a list of valid values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The first date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format and use Coordinated Universal Time (UTC), for example: 2019-07-19T20:00:00Z for 8:00 PM UTC July 19, 2019. This value should also be fewer than 90 days from the current day.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignDateRangeKpiResponse {
    #[serde(rename = "CampaignDateRangeKpiResponse")]
    pub campaign_date_range_kpi_response: CampaignDateRangeKpiResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignVersionRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The unique version number (Version property) for the campaign version.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignVersionResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignVersionsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignVersionsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCampaignsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCampaignsResponse {
    #[serde(rename = "CampaignsResponse")]
    pub campaigns_response: CampaignsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChannelsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetChannelsResponse {
    #[serde(rename = "ChannelsResponse")]
    pub channels_response: ChannelsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailTemplateResponse {
    #[serde(rename = "EmailTemplateResponse")]
    pub email_template_response: EmailTemplateResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEndpointRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    pub endpoint_response: EndpointResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventStreamRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetExportJobRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    pub export_job_response: ExportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetExportJobsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGcmChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImportJobRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    pub import_job_response: ImportJobResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetImportJobsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJourneyDateRangeKpiRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The last date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format, for example: 2019-07-19T00:00:00Z for July 19, 2019 and 2019-07-19T20:00:00Z for 8:00 PM July 19, 2019.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, to retrieve data for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. Examples are email-open-rate and successful-delivery-rate. For a list of valid values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The first date and time to retrieve data for, as part of an inclusive date range that filters the query results. This value should be in extended ISO 8601 format, for example: 2019-07-15T00:00:00Z for July 15, 2019 and 2019-07-15T16:00:00Z for 4:00 PM July 15, 2019.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJourneyDateRangeKpiResponse {
    #[serde(rename = "JourneyDateRangeKpiResponse")]
    pub journey_date_range_kpi_response: JourneyDateRangeKpiResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJourneyExecutionActivityMetricsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey activity.</p>
    #[serde(rename = "JourneyActivityId")]
    pub journey_activity_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJourneyExecutionActivityMetricsResponse {
    #[serde(rename = "JourneyExecutionActivityMetricsResponse")]
    pub journey_execution_activity_metrics_response: JourneyExecutionActivityMetricsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJourneyExecutionMetricsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJourneyExecutionMetricsResponse {
    #[serde(rename = "JourneyExecutionMetricsResponse")]
    pub journey_execution_metrics_response: JourneyExecutionMetricsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJourneyRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    pub journey_response: JourneyResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPushTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPushTemplateResponse {
    #[serde(rename = "PushNotificationTemplateResponse")]
    pub push_notification_template_response: PushNotificationTemplateResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentExportJobsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    pub export_jobs_response: ExportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentImportJobsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    pub import_jobs_response: ImportJobsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentVersionRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The unique version number (Version property) for the campaign version.</p>
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentVersionResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentVersionsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentVersionsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSegmentsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSegmentsResponse {
    #[serde(rename = "SegmentsResponse")]
    pub segments_response: SegmentsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSmsChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSmsTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSmsTemplateResponse {
    #[serde(rename = "SMSTemplateResponse")]
    pub sms_template_response: SMSTemplateResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetUserEndpointsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the user.</p>
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    pub endpoints_response: EndpointsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVoiceTemplateRequest {
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVoiceTemplateResponse {
    #[serde(rename = "VoiceTemplateResponse")]
    pub voice_template_response: VoiceTemplateResponse,
}

/// <p>Specifies the settings for a holdout activity in a journey. This type of activity stops a journey for a specified percentage of participants.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoldoutActivity {
    /// <p>The unique identifier for the next activity to perform, after performing the holdout activity.</p>
    #[serde(rename = "NextActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    /// <p>The percentage of participants who shouldn't continue the journey.</p> <p>To determine which participants are held out, Amazon Pinpoint applies a probability-based algorithm to the percentage that you specify. Therefore, the actual percentage of participants who are held out may not be equal to the percentage that you specify.</p>
    #[serde(rename = "Percentage")]
    pub percentage: i64,
}

/// <p>Specifies the settings for a job that imports endpoint definitions from an Amazon Simple Storage Service (Amazon S3) bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportJobRequest {
    /// <p>Specifies whether to create a segment that contains the endpoints, when the endpoint definitions are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to an external ID key in an IAM trust policy. Amazon Pinpoint previously used this value to assume an IAM role when importing endpoint definitions, but we removed this requirement. We don't recommend use of external IDs for IAM roles that are assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions to import. Valid values are: CSV, for comma-separated values format; and, JSON, for newline-delimited JSON format. If the Amazon S3 location stores multiple files that use different formats, Amazon Pinpoint imports data only from the files that use the specified format.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>Specifies whether to register the endpoints with Amazon Pinpoint, when the endpoint definitions are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that authorizes Amazon Pinpoint to access the Amazon S3 location to import endpoint definitions from.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The URL of the Amazon Simple Storage Service (Amazon S3) bucket that contains the endpoint definitions to import. This location can be a folder or a single file. If the location is a folder, Amazon Pinpoint imports endpoint definitions from the files in this location, including any subfolders that the folder contains.</p> <p>The URL should be in the following format: s3://<replaceable>bucket-name</replaceable>/<replaceable>folder-name</replaceable>/<replaceable>file-name</replaceable>. The location can end with the key for an individual object or a prefix that qualifies multiple objects.</p>
    #[serde(rename = "S3Url")]
    pub s3_url: String,
    /// <p>The identifier for the segment to update or add the imported endpoint definitions to, if the import job is meant to update an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The custom name for the segment that's created by the import job, if the value of the DefineSegment property is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Provides information about the resource settings for a job that imports endpoint definitions from one or more files. The files can be stored in an Amazon Simple Storage Service (Amazon S3) bucket or uploaded directly from a computer by using the Amazon Pinpoint console.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportJobResource {
    /// <p>Specifies whether the import job creates a segment that contains the endpoints, when the endpoint definitions are imported.</p>
    #[serde(rename = "DefineSegment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to an external ID key in an IAM trust policy. Amazon Pinpoint previously used this value to assume an IAM role when importing endpoint definitions, but we removed this requirement. We don't recommend use of external IDs for IAM roles that are assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The format of the files that contain the endpoint definitions to import. Valid values are: CSV, for comma-separated values format; and, JSON, for newline-delimited JSON format.</p> <p>If the files are stored in an Amazon S3 location and that location contains multiple files that use different formats, Amazon Pinpoint imports data only from the files that use the specified format.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>Specifies whether the import job registers the endpoints with Amazon Pinpoint, when the endpoint definitions are imported.</p>
    #[serde(rename = "RegisterEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that authorizes Amazon Pinpoint to access the Amazon S3 location to import endpoint definitions from.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The URL of the Amazon Simple Storage Service (Amazon S3) bucket that contains the endpoint definitions to import. This location can be a folder or a single file. If the location is a folder, Amazon Pinpoint imports endpoint definitions from the files in this location, including any subfolders that the folder contains.</p> <p>The URL should be in the following format: s3://<replaceable>bucket-name</replaceable>/<replaceable>folder-name</replaceable>/<replaceable>file-name</replaceable>. The location can end with the key for an individual object or a prefix that qualifies multiple objects.</p>
    #[serde(rename = "S3Url")]
    pub s3_url: String,
    /// <p>The identifier for the segment that the import job updates or adds endpoint definitions to, if the import job updates an existing segment.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The custom name for the segment that's created by the import job, if the value of the DefineSegment property is true.</p>
    #[serde(rename = "SegmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

/// <p>Provides information about the status and settings of a job that imports endpoint definitions from one or more files. The files can be stored in an Amazon Simple Storage Service (Amazon S3) bucket or uploaded directly from a computer by using the Amazon Pinpoint console.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportJobResponse {
    /// <p>The unique identifier for the application that's associated with the import job.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The number of pieces that were processed successfully (completed) by the import job, as of the time of the request.</p>
    #[serde(rename = "CompletedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i64>,
    /// <p>The date, in ISO 8601 format, when the import job was completed.</p>
    #[serde(rename = "CompletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// <p>The date, in ISO 8601 format, when the import job was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The resource settings that apply to the import job.</p>
    #[serde(rename = "Definition")]
    pub definition: ImportJobResource,
    /// <p>The number of pieces that weren't processed successfully (failed) by the import job, as of the time of the request.</p>
    #[serde(rename = "FailedPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i64>,
    /// <p>An array of entries, one for each of the first 100 entries that weren't processed successfully (failed) by the import job, if any.</p>
    #[serde(rename = "Failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    /// <p>The unique identifier for the import job.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The status of the import job. The job status is FAILED if Amazon Pinpoint wasn't able to process one or more pieces in the job.</p>
    #[serde(rename = "JobStatus")]
    pub job_status: String,
    /// <p>The total number of endpoint definitions that weren't processed successfully (failed) by the import job, typically because an error, such as a syntax error, occurred.</p>
    #[serde(rename = "TotalFailures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i64>,
    /// <p>The total number of pieces that must be processed to complete the import job. Each piece consists of an approximately equal portion of the endpoint definitions that are part of the import job.</p>
    #[serde(rename = "TotalPieces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i64>,
    /// <p>The total number of endpoint definitions that were processed by the import job.</p>
    #[serde(rename = "TotalProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i64>,
    /// <p>The job type. This value is IMPORT for import jobs.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Provides information about the status and settings of all the import jobs that are associated with an application or segment. An import job is a job that imports endpoint definitions from one or more files.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportJobsResponse {
    /// <p>An array of responses, one for each import job that's associated with the application (Import Jobs resource) or segment (Segment Import Jobs resource).</p>
    #[serde(rename = "Item")]
    pub item: Vec<ImportJobResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Provides information about the results of a request to create or update an endpoint that's associated with an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ItemResponse {
    /// <p>The response that was received after the endpoint data was accepted.</p>
    #[serde(rename = "EndpointItemResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_item_response: Option<EndpointItemResponse>,
    /// <p>A multipart response object that contains a key and a value for each event in the request. In each object, the event ID is the key and an EventItemResponse object is the value.</p>
    #[serde(rename = "EventsItemResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_item_response: Option<::std::collections::HashMap<String, EventItemResponse>>,
}

/// <p>Provides the results of a query that retrieved the data for a standard engagement metric that applies to a journey, and provides information about that query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JourneyDateRangeKpiResponse {
    /// <p>The unique identifier for the application that the metric applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The last date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>The unique identifier for the journey that the metric applies to.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The name of the metric, also referred to as a <i>key performance indicator (KPI)</i>, that the data was retrieved for. This value describes the associated metric and consists of two or more terms, which are comprised of lowercase alphanumeric characters, separated by a hyphen. For a list of possible values, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "KpiName")]
    pub kpi_name: String,
    /// <p>An array of objects that contains the results of the query. Each object contains the value for the metric and metadata about that value.</p>
    #[serde(rename = "KpiResult")]
    pub kpi_result: BaseKpiResult,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null for the Journey Engagement Metrics resource because the resource returns all results in a single page.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The first date and time of the date range that was used to filter the query results, in extended ISO 8601 format. The date range is inclusive.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

/// <p>Specifies the "From" address for an email message that's sent to participants in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JourneyEmailMessage {
    /// <p>The verified email address to send the email message from. The default address is the FromAddress specified for the email channel for the application.</p>
    #[serde(rename = "FromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
}

/// <p>Provides the results of a query that retrieved the data for a standard execution metric that applies to a journey activity, and provides information about that query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JourneyExecutionActivityMetricsResponse {
    /// <p><p>The type of activity that the metric applies to. Possible values are:</p> <ul><li><p>CONDITIONAL<em>SPLIT - For a yes/no split activity, which is an activity that sends participants down one of two paths in a journey.</p></li> <li><p>HOLDOUT - For a holdout activity, which is an activity that stops a journey for a specified percentage of participants.</p></li> <li><p>MESSAGE - For an email activity, which is an activity that sends an email message to participants.</p></li> <li><p>MULTI</em>CONDITIONAL<em>SPLIT - For a multivariate split activity, which is an activity that sends participants down one of as many as five paths in a journey.</p></li> <li><p>RANDOM</em>SPLIT - For a random split activity, which is an activity that sends specified percentages of participants down one of as many as five paths in a journey.</p></li> <li><p>WAIT - For a wait activity, which is an activity that waits for a certain amount of time or until a specific date and time before moving participants to the next activity in a journey.</p></li></ul></p>
    #[serde(rename = "ActivityType")]
    pub activity_type: String,
    /// <p>The unique identifier for the application that the metric applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the activity that the metric applies to.</p>
    #[serde(rename = "JourneyActivityId")]
    pub journey_activity_id: String,
    /// <p>The unique identifier for the journey that the metric applies to.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The date and time, in ISO 8601 format, when Amazon Pinpoint last evaluated the execution status of the activity and updated the data for the metric.</p>
    #[serde(rename = "LastEvaluatedTime")]
    pub last_evaluated_time: String,
    /// <p>A JSON object that contains the results of the query. The results vary depending on the type of activity (ActivityType). For information about the structure and contents of the results, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "Metrics")]
    pub metrics: ::std::collections::HashMap<String, String>,
}

/// <p>Provides the results of a query that retrieved the data for a standard execution metric that applies to a journey, and provides information about that query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JourneyExecutionMetricsResponse {
    /// <p>The unique identifier for the application that the metric applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey that the metric applies to.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    /// <p>The date and time, in ISO 8601 format, when Amazon Pinpoint last evaluated the journey and updated the data for the metric.</p>
    #[serde(rename = "LastEvaluatedTime")]
    pub last_evaluated_time: String,
    /// <p>A JSON object that contains the results of the query. For information about the structure and contents of the results, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon Pinpoint Developer Guide</a>.</p>
    #[serde(rename = "Metrics")]
    pub metrics: ::std::collections::HashMap<String, String>,
}

/// <p>Specifies limits on the messages that a journey can send and the number of times participants can enter a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JourneyLimits {
    /// <p>The maximum number of messages that the journey can send to a single participant during a 24-hour period. The maximum value is 100.</p>
    #[serde(rename = "DailyCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i64>,
    /// <p>The maximum number of times that a participant can enter the journey. The maximum value is 100. To allow participants to enter the journey an unlimited number of times, set this value to 0.</p>
    #[serde(rename = "EndpointReentryCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_reentry_cap: Option<i64>,
    /// <p>The maximum number of messages that the journey can send each second.</p>
    #[serde(rename = "MessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i64>,
}

/// <p>Provides information about the status, configuration, and other settings for a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JourneyResponse {
    /// <p>A map that contains a set of Activity objects, one object for each activity in the journey. For each Activity object, the key is the unique identifier (string) for an activity and the value is the settings for the activity.</p>
    #[serde(rename = "Activities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<::std::collections::HashMap<String, Activity>>,
    /// <p>The unique identifier for the application that the journey applies to.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The date, in ISO 8601 format, when the journey was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The date, in ISO 8601 format, when the journey was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The messaging and entry limits for the journey.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<JourneyLimits>,
    /// <p>Specifies whether the journey's scheduled start and end times use each participant's local time. If this value is true, the schedule uses each participant's local time.</p>
    #[serde(rename = "LocalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<bool>,
    /// <p>The name of the journey.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The quiet time settings for the journey. Quiet time is a specific time range when a journey doesn't send messages to participants, if all the following conditions are met:</p> <ul><li><p>The EndpointDemographic.Timezone property of the endpoint for the participant is set to a valid value.</p></li> <li><p>The current time in the participant's time zone is later than or equal to the time specified by the QuietTime.Start property for the journey.</p></li> <li><p>The current time in the participant's time zone is earlier than or equal to the time specified by the QuietTime.End property for the journey.</p></li></ul> <p>If any of the preceding conditions isn't met, the participant will receive messages from the journey, even if quiet time is enabled.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    /// <p>The frequency with which Amazon Pinpoint evaluates segment and event data for the journey, as a duration in ISO 8601 format.</p>
    #[serde(rename = "RefreshFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_frequency: Option<String>,
    /// <p>The schedule settings for the journey.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JourneySchedule>,
    /// <p>The unique identifier for the first activity in the journey.</p>
    #[serde(rename = "StartActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_activity: Option<String>,
    /// <p>The segment that defines which users are participants in the journey.</p>
    #[serde(rename = "StartCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_condition: Option<StartCondition>,
    /// <p><p>The current status of the journey. Possible values are:</p> <ul><li><p>DRAFT - The journey is being developed and hasn&#39;t been published yet.</p></li> <li><p>ACTIVE - The journey has been developed and published. Depending on the journey&#39;s schedule, the journey may currently be running or scheduled to start running at a later time. If a journey&#39;s status is ACTIVE, you can&#39;t add, change, or remove activities from it.</p></li> <li><p>COMPLETED - The journey has been published and has finished running. All participants have entered the journey and no participants are waiting to complete the journey or any activities in the journey.</p></li> <li><p>CANCELLED - The journey has been stopped. If a journey&#39;s status is CANCELLED, you can&#39;t add, change, or remove activities or segment settings from the journey.</p></li> <li><p>CLOSED - The journey has been published and has started running. It may have also passed its scheduled end time, or passed its scheduled start time and a refresh frequency hasn&#39;t been specified for it. If a journey&#39;s status is CLOSED, you can&#39;t add participants to it, and no existing participants can enter the journey for the first time. However, any existing participants who are currently waiting to start an activity may continue the journey.</p></li></ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the journey. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the schedule settings for a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JourneySchedule {
    /// <p>The scheduled time, in ISO 8601 format, when the journey ended or will end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The scheduled time, in ISO 8601 format, when the journey began or will begin.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The starting UTC offset for the journey schedule, if the value of the journey's LocalTime property is true. Valid values are: UTC,
    /// UTC+01, UTC+02, UTC+03, UTC+03:30, UTC+04, UTC+04:30, UTC+05, UTC+05:30,
    /// UTC+05:45, UTC+06, UTC+06:30, UTC+07, UTC+08, UTC+08:45, UTC+09, UTC+09:30,
    /// UTC+10, UTC+10:30, UTC+11, UTC+12, UTC+12:45, UTC+13, UTC+13:45, UTC-02,
    /// UTC-02:30, UTC-03, UTC-03:30, UTC-04, UTC-05, UTC-06, UTC-07, UTC-08, UTC-09,
    /// UTC-09:30, UTC-10, and UTC-11.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Changes the status of a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct JourneyStateRequest {
    /// <p>The status of the journey. Currently, the only supported value is CANCELLED.</p> <p>If you cancel a journey, Amazon Pinpoint continues to perform activities that are currently in progress, until those activities are complete. Amazon Pinpoint also continues to collect and aggregate analytics data for those activities, until they are complete, and any activities that were complete when you cancelled the journey.</p> <p>After you cancel a journey, you can't add, change, or remove any activities from the journey. In addition, Amazon Pinpoint stops evaluating the journey and doesn't perform any activities that haven't started.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Provides information about the status, configuration, and other settings for all the journeys that are associated with an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JourneysResponse {
    /// <p>An array of responses, one for each journey that's associated with the application.</p>
    #[serde(rename = "Item")]
    pub item: Vec<JourneyResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJourneysRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJourneysResponse {
    #[serde(rename = "JourneysResponse")]
    pub journeys_response: JourneysResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "TagsModel")]
    pub tags_model: TagsModel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplateVersionsRequest {
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. Valid values are: EMAIL, PUSH, SMS, and VOICE.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTemplateVersionsResponse {
    #[serde(rename = "TemplateVersionsResponse")]
    pub template_versions_response: TemplateVersionsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTemplatesRequest {
    /// <p>The  string that specifies which page of results to return in a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is currently not supported for application, campaign, and journey metrics.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// <p>The substring to match in the names of the message templates to include in the results. If you specify this value, Amazon Pinpoint returns only those templates whose names begin with the value that you specify.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The type of message template to include in the results. Valid values are: EMAIL, PUSH, SMS, and VOICE. To include all types of templates in the results, don't include this parameter in your request.</p>
    #[serde(rename = "TemplateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTemplatesResponse {
    #[serde(rename = "TemplatesResponse")]
    pub templates_response: TemplatesResponse,
}

/// <p>Specifies the content and settings for a push notification that's sent to recipients of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// <p><p>The action to occur if a recipient taps the push notification. Valid values are:</p> <ul><li><p>OPEN<em>APP - Your app opens or it becomes the foreground app if it was sent to the background. This is the default action.</p></li> <li><p>DEEP</em>LINK - Your app opens and displays a designated user interface in the app. This setting uses the deep-linking features of iOS and Android.</p></li> <li><p>URL - The default mobile browser on the recipient&#39;s device opens and loads the web page at a URL that you specify.</p></li></ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The body of the notification message. The maximum number of characters is 200.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The URL of the image to display as the push-notification icon, such as the icon for the app.</p>
    #[serde(rename = "ImageIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    /// <p>The URL of the image to display as the small, push-notification icon, such as a small version of the icon for the app.</p>
    #[serde(rename = "ImageSmallIconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_small_icon_url: Option<String>,
    /// <p>The URL of an image to display in the push notification.</p>
    #[serde(rename = "ImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The JSON payload to use for a silent push notification.</p>
    #[serde(rename = "JsonBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<String>,
    /// <p>The URL of the image or video to display in the push notification.</p>
    #[serde(rename = "MediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// <p>The raw, JSON-formatted string to use as the payload for the notification message. If specified, this value overrides all other content for the message.</p>
    #[serde(rename = "RawContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    /// <p>Specifies whether the notification is a silent push notification, which is a push notification that doesn't display on a recipient's device. Silent push notifications can be used for cases such as updating an app's configuration, displaying messages in an in-app message center, or supporting phone home functionality.</p>
    #[serde(rename = "SilentPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    /// <p>The number of seconds that the push-notification service should keep the message, if the service is unable to deliver the notification the first time. This value is converted to an expiration value when it's sent to a push-notification service. If this value is 0, the service treats the notification as if it expires immediately and the service doesn't store or try to deliver the notification again.</p> <p>This value doesn't apply to messages that are sent through the Amazon Device Messaging (ADM) service.</p>
    #[serde(rename = "TimeToLive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i64>,
    /// <p>The title to display above the notification message on a recipient's device.</p>
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The URL to open in a recipient's default mobile browser, if a recipient taps the push notification and the value of the Action property is URL.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Provides information about an API request or response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MessageBody {
    /// <p>The message that's returned from the API.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The unique identifier for the request or response.</p>
    #[serde(rename = "RequestID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Specifies the message configuration settings for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageConfiguration {
    /// <p>The message that the campaign sends through the ADM (Amazon Device Messaging) channel. This message overrides the default message.</p>
    #[serde(rename = "ADMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm_message: Option<Message>,
    /// <p>The message that the campaign sends through the APNs (Apple Push Notification service) channel. This message overrides the default message.</p>
    #[serde(rename = "APNSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_message: Option<Message>,
    /// <p>The message that the campaign sends through the Baidu (Baidu Cloud Push) channel. This message overrides the default message.</p>
    #[serde(rename = "BaiduMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<Message>,
    /// <p>The default message that the campaign sends through all the channels that are configured for the campaign.</p>
    #[serde(rename = "DefaultMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<Message>,
    /// <p>The message that the campaign sends through the email channel.</p>
    #[serde(rename = "EmailMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<CampaignEmailMessage>,
    /// <p>The message that the campaign sends through the GCM channel, which enables Amazon Pinpoint to send push notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. This message overrides the default message.</p>
    #[serde(rename = "GCMMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_message: Option<Message>,
    /// <p>The message that the campaign sends through the SMS channel.</p>
    #[serde(rename = "SMSMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<CampaignSmsMessage>,
}

/// <p>Specifies the configuration and other settings for a message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageRequest {
    /// <p>A map of key-value pairs, where each key is an address and each value is an AddressConfiguration object. An address can be a push notification token, a phone number, or an email address. You can use an AddressConfiguration object to tailor the message for an address by specifying settings such as content overrides and message variables.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<::std::collections::HashMap<String, AddressConfiguration>>,
    /// <p>A map of custom attributes to attach to the message. For a push notification, this payload is added to the data.pinpoint object. For an email or text message, this payload is added to email/SMS delivery receipt event attributes.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map of key-value pairs, where each key is an endpoint ID and each value is an EndpointSendConfiguration object. You can use an EndpointSendConfiguration object to tailor the message for an endpoint by specifying settings such as content overrides and message variables.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, EndpointSendConfiguration>>,
    /// <p>The settings and content for the default message and any default messages that you defined for specific channels.</p>
    #[serde(rename = "MessageConfiguration")]
    pub message_configuration: DirectMessageConfiguration,
    /// <p>The message template to use for the message.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>The unique identifier for tracing the message. This identifier is visible to message recipients.</p>
    #[serde(rename = "TraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

/// <p>Provides information about the results of a request to send a message to an endpoint address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MessageResponse {
    /// <p>The unique identifier for the application that was used to send the message.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A map that contains a multipart response for each address that the message was sent to. In the map, the endpoint ID is the key and the result is the value.</p>
    #[serde(rename = "EndpointResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_result: Option<::std::collections::HashMap<String, EndpointMessageResult>>,
    /// <p>The identifier for the original request that the message was delivered for.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>A map that contains a multipart response for each address (email address, phone number, or push notification token) that the message was sent to. In the map, the address is the key and the result is the value.</p>
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<::std::collections::HashMap<String, MessageResult>>,
}

/// <p>Provides information about the results of sending a message directly to an endpoint address.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MessageResult {
    /// <p><p>The delivery status of the message. Possible values are:</p> <ul> <li><p>DUPLICATE - The endpoint address is a duplicate of another endpoint address. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li>   <li><p>OPT<em>OUT - The user who&#39;s associated with the endpoint address has opted out of receiving messages from you. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li> <li><p>PERMANENT</em>FAILURE - An error occurred when delivering the message to the endpoint address. Amazon Pinpoint won&#39;t attempt to send the message again.</p></li>   <li><p>SUCCESSFUL - The message was successfully delivered to the endpoint address.</p></li> <li><p>TEMPORARY<em>FAILURE - A temporary error occurred. Amazon Pinpoint will attempt to deliver the message again later.</p></li> <li><p>THROTTLED - Amazon Pinpoint throttled the operation to send the message to the endpoint address.</p></li> <li><p>TIMEOUT - The message couldn&#39;t be sent within the timeout period.</p></li> <li><p>UNKNOWN</em>FAILURE - An unknown error occurred.</p></li></ul></p>
    #[serde(rename = "DeliveryStatus")]
    pub delivery_status: String,
    /// <p>The unique identifier for the message that was sent.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// <p>The downstream service status code for delivering the message.</p>
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
    /// <p>The status message for delivering the message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>For push notifications that are sent through the GCM channel, specifies whether the endpoint's device registration token was updated as part of delivering the message.</p>
    #[serde(rename = "UpdatedToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

/// <p>Specifies metric-based criteria for including or excluding endpoints from a segment. These criteria derive from custom metrics that you define for endpoints.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDimension {
    /// <p>The operator to use when comparing metric values. Valid values are: GREATER_THAN, LESS_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, and EQUAL.</p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// <p>The value to compare.</p>
    #[serde(rename = "Value")]
    pub value: f64,
}

/// <p>Specifies a condition to evaluate for an activity path in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiConditionalBranch {
    /// <p>The condition to evaluate for the activity path.</p>
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<SimpleCondition>,
    /// <p>The unique identifier for the next activity to perform, after completing the activity for the path.</p>
    #[serde(rename = "NextActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
}

/// <p>Specifies the settings for a multivariate split activity in a journey. This type of activity sends participants down one of as many as five paths (including a default <i>Else</i> path) in a journey, based on conditions that you specify.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiConditionalSplitActivity {
    /// <p>The paths for the activity, including the conditions for entering each path and the activity to perform for each path.</p>
    #[serde(rename = "Branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<MultiConditionalBranch>>,
    /// <p>The unique identifier for the activity to perform for participants who don't meet any of the conditions specified for other paths in the activity.</p>
    #[serde(rename = "DefaultActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_activity: Option<String>,
    /// <p>The amount of time to wait or the date and time when Amazon Pinpoint determines whether the conditions are met.</p>
    #[serde(rename = "EvaluationWaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_wait_time: Option<WaitTime>,
}

/// <p>Specifies a phone number to validate and retrieve information about.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NumberValidateRequest {
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "IsoCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    /// <p>The phone number to retrieve information about. The phone number that you provide should include a valid numeric country code. Otherwise, the operation might result in an error.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// <p>Provides information about a phone number.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NumberValidateResponse {
    /// <p>The carrier or service provider that the phone number is currently registered with. In some countries and regions, this value may be the carrier or service provider that the phone number was originally registered with.</p>
    #[serde(rename = "Carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// <p>The name of the city where the phone number was originally registered.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The cleansed phone number, in E.164 format, for the location where the phone number was originally registered.</p>
    #[serde(rename = "CleansedPhoneNumberE164")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_e164: Option<String>,
    /// <p>The cleansed phone number, in the format for the location where the phone number was originally registered.</p>
    #[serde(rename = "CleansedPhoneNumberNational")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_national: Option<String>,
    /// <p>The name of the country or region where the phone number was originally registered.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_iso_2: Option<String>,
    /// <p>The numeric code for the country or region where the phone number was originally registered.</p>
    #[serde(rename = "CountryCodeNumeric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_numeric: Option<String>,
    /// <p>The name of the county where the phone number was originally registered.</p>
    #[serde(rename = "County")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, that was sent in the request body.</p>
    #[serde(rename = "OriginalCountryCodeIso2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_country_code_iso_2: Option<String>,
    /// <p>The phone number that was sent in the request body.</p>
    #[serde(rename = "OriginalPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_phone_number: Option<String>,
    /// <p>The description of the phone type. Valid values are: MOBILE, LANDLINE, VOIP,
    /// INVALID, PREPAID, and OTHER.</p>
    #[serde(rename = "PhoneType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    /// <p>The phone type, represented by an integer. Valid values are: 0 (mobile), 1 (landline), 2 (VoIP), 3 (invalid), 4 (other), and 5 (prepaid).</p>
    #[serde(rename = "PhoneTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type_code: Option<i64>,
    /// <p>The time zone for the location where the phone number was originally registered.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The postal or ZIP code for the location where the phone number was originally registered.</p>
    #[serde(rename = "ZipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PhoneNumberValidateRequest {
    #[serde(rename = "NumberValidateRequest")]
    pub number_validate_request: NumberValidateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PhoneNumberValidateResponse {
    #[serde(rename = "NumberValidateResponse")]
    pub number_validate_response: NumberValidateResponse,
}

/// <p>Specifies the properties and attributes of an endpoint that's associated with an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PublicEndpoint {
    /// <p>The unique identifier for the recipient, such as a device token, email address, or mobile phone number.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>One or more custom attributes that describe the endpoint by associating a name with an array of values. You can use these attributes as filter criteria when you create segments.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The channel that's used when sending messages or push notifications to the endpoint.</p>
    #[serde(rename = "ChannelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    /// <p>The demographic information for the endpoint, such as the time zone and platform.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    /// <p>The date and time, in ISO 8601 format, when the endpoint was last updated.</p>
    #[serde(rename = "EffectiveDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// <p>Specifies whether to send messages or push notifications to the endpoint. Valid values are: ACTIVE, messages are sent to the endpoint; and, INACTIVE, messages aren’t sent to the endpoint.</p> <p>Amazon Pinpoint automatically sets this value to ACTIVE when you create an endpoint or update an existing endpoint. Amazon Pinpoint automatically sets this value to INACTIVE if you update another endpoint that has the same address specified by the Address property.</p>
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    /// <p>The geographic information for the endpoint.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    /// <p>One or more custom metrics that your app reports to Amazon Pinpoint for the endpoint.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>Specifies whether the user who's associated with the endpoint has opted out of receiving messages and push notifications from you. Possible values are: ALL, the user has opted out and doesn't want to receive any messages or push notifications; and, NONE, the user hasn't opted out and wants to receive all messages and push notifications.</p>
    #[serde(rename = "OptOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    /// <p>A unique identifier that's generated each time the endpoint is updated.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>One or more custom user attributes that your app reports to Amazon Pinpoint for the user who's associated with the endpoint.</p>
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

/// <p>Specifies the content and settings for a message template that can be used in messages that are sent through a push notification channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PushNotificationTemplateRequest {
    /// <p>The message template to use for the ADM (Amazon Device Messaging) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "ADM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<AndroidPushNotificationTemplate>,
    /// <p>The message template to use for the APNs (Apple Push Notification service) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "APNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<APNSPushNotificationTemplate>,
    /// <p>The message template to use for the Baidu (Baidu Cloud Push) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "Baidu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu: Option<AndroidPushNotificationTemplate>,
    /// <p>The default message template to use for push notification channels.</p>
    #[serde(rename = "Default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultPushNotificationTemplate>,
    /// <p>A JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The message template to use for the GCM channel, which is used to send notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "GCM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm: Option<AndroidPushNotificationTemplate>,
    /// <p>A custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about the content and settings for a message template that can be used in messages that are sent through a push notification channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PushNotificationTemplateResponse {
    /// <p>The message template that's used for the ADM (Amazon Device Messaging) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "ADM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<AndroidPushNotificationTemplate>,
    /// <p>The message template that's used for the APNs (Apple Push Notification service) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "APNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<APNSPushNotificationTemplate>,
    /// <p>The Amazon Resource Name (ARN) of the message template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The message template that's used for the Baidu (Baidu Cloud Push) channel. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "Baidu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu: Option<AndroidPushNotificationTemplate>,
    /// <p>The date, in ISO 8601 format, when the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The default message template that's used for push notification channels.</p>
    #[serde(rename = "Default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultPushNotificationTemplate>,
    /// <p>The JSON object that specifies the default values that are used for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The message template that's used for the GCM channel, which is used to send notifications through the Firebase Cloud Messaging (FCM), formerly Google Cloud Messaging (GCM), service. This message template overrides the default template for push notification channels (DefaultPushNotificationTemplate).</p>
    #[serde(rename = "GCM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm: Option<AndroidPushNotificationTemplate>,
    /// <p>The date, in ISO 8601 format, when the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. For a push notification template, this value is PUSH.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The unique identifier, as an integer, for the active version of the message template, or the version of the template that you specified by using the version parameter in your request.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventStreamRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteEventStream")]
    pub write_event_stream: WriteEventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventStreamResponse {
    #[serde(rename = "EventStream")]
    pub event_stream: EventStream,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EventsRequest")]
    pub events_request: EventsRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventsResponse {
    #[serde(rename = "EventsResponse")]
    pub events_response: EventsResponse,
}

/// <p>Specifies the start and end times that define a time range when messages aren't sent to endpoints.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuietTime {
    /// <p>The specific time when quiet time ends. This value has to use 24-hour notation and be in HH:MM format, where HH is the hour (with a leading zero, if applicable) and MM is the minutes. For example, use 02:30 to represent 2:30 AM, or 14:30 to represent 2:30 PM.</p>
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>The specific time when quiet time begins. This value has to use 24-hour notation and be in HH:MM format, where HH is the hour (with a leading zero, if applicable) and MM is the minutes. For example, use 02:30 to represent 2:30 AM, or 14:30 to represent 2:30 PM.</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// <p>Specifies the settings for a random split activity in a journey. This type of activity randomly sends specified percentages of participants down one of as many as five paths in a journey, based on conditions that you specify.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomSplitActivity {
    /// <p>The paths for the activity, including the percentage of participants to enter each path and the activity to perform for each path.</p>
    #[serde(rename = "Branches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<RandomSplitEntry>>,
}

/// <p>Specifies the settings for a path in a random split activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomSplitEntry {
    /// <p>The unique identifier for the next activity to perform, after completing the activity for the path.</p>
    #[serde(rename = "NextActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    /// <p>The percentage of participants to send down the activity path.</p> <p>To determine which participants are sent down each path, Amazon Pinpoint applies a probability-based algorithm to the percentages that you specify for the paths. Therefore, the actual percentage of participants who are sent down a path may not be equal to the percentage that you specify.</p>
    #[serde(rename = "Percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i64>,
}

/// <p>Specifies the contents of an email message, represented as a raw MIME message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RawEmail {
    /// <p>The email message, represented as a raw MIME message. The entire message must be base64 encoded.</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bytes::Bytes>,
}

/// <p>Specifies criteria for including or excluding endpoints from a segment based on how recently an endpoint was active.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecencyDimension {
    /// <p>The duration to use when determining whether an endpoint is active or inactive.</p>
    #[serde(rename = "Duration")]
    pub duration: String,
    /// <p>The type of recency dimension to use for the segment. Valid values are: ACTIVE, endpoints that were active within the specified duration are included in the segment; and, INACTIVE, endpoints that weren't active within the specified duration are included in the segment.</p>
    #[serde(rename = "RecencyType")]
    pub recency_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveAttributesRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p><p>The type of attribute or attributes to remove. Valid values are:</p> <ul><li><p>endpoint-custom-attributes - Custom attributes that describe endpoints, such as the date when an associated user opted in or out of receiving communications from you through a specific type of channel.</p></li> <li><p>endpoint-metric-attributes - Custom metrics that your app reports to Amazon Pinpoint for endpoints, such as the number of app sessions or the number of items left in a cart.</p></li> <li><p>endpoint-user-attributes - Custom attributes that describe users, such as first name, last name, and age.</p></li></ul></p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    #[serde(rename = "UpdateAttributesRequest")]
    pub update_attributes_request: UpdateAttributesRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveAttributesResponse {
    #[serde(rename = "AttributesResource")]
    pub attributes_resource: AttributesResource,
}

/// <p>Provides the results of a query that retrieved the data for a standard metric that applies to an application, campaign, or journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResultRow {
    /// <p>An array of objects that defines the field and field values that were used to group data in a result set that contains multiple results. This value is null if the data in a result set isn’t grouped.</p>
    #[serde(rename = "GroupedBys")]
    pub grouped_bys: Vec<ResultRowValue>,
    /// <p>An array of objects that provides pre-aggregated values for a standard metric that applies to an application, campaign, or journey.</p>
    #[serde(rename = "Values")]
    pub values: Vec<ResultRowValue>,
}

/// <p>Provides a single value and metadata about that value as part of an array of query results for a standard metric that applies to an application, campaign, or journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResultRowValue {
    /// <p>The friendly name of the metric whose value is specified by the Value property.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The data type of the value specified by the Value property.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>In a Values object, the value for the metric that the query retrieved data for. In a GroupedBys object, the value for the field that was used to group data in a result set that contains multiple results (Values objects).</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Specifies the status and settings of the SMS channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SMSChannelRequest {
    /// <p>Specifies whether to enable the SMS channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The identity that you want to display on recipients' devices when they receive messages from the SMS channel.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>The registered short code that you want to use when you send messages through the SMS channel.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
}

/// <p>Provides information about the status and settings of the SMS channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SMSChannelResponse {
    /// <p>The unique identifier for the application that the SMS channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the SMS channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Specifies whether the SMS channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the SMS channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the SMS channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the SMS channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the SMS channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the SMS channel, this value is SMS.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The maximum number of promotional messages that you can send through the SMS channel each second.</p>
    #[serde(rename = "PromotionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_messages_per_second: Option<i64>,
    /// <p>The identity that displays on recipients' devices when they receive messages from the SMS channel.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>The registered short code to use when you send messages through the SMS channel.</p>
    #[serde(rename = "ShortCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    /// <p>The maximum number of transactional messages that you can send through the SMS channel each second.</p>
    #[serde(rename = "TransactionalMessagesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_messages_per_second: Option<i64>,
    /// <p>The current version of the SMS channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the default settings for a one-time SMS message that's sent directly to an endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SMSMessage {
    /// <p>The body of the SMS message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The SMS program name that you provided to AWS Support when you requested your dedicated number.</p>
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// <p>The SMS message type. Valid values are: TRANSACTIONAL, the message is critical or time-sensitive, such as a one-time password that supports a customer transaction; and, PROMOTIONAL, the message is not critical or time-sensitive, such as a marketing message.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The number to send the SMS message from. This value should be one of the dedicated long or short codes that's assigned to your AWS account. If you don't specify a long or short code, Amazon Pinpoint assigns a random long code to the SMS message and sends the message from that code.</p>
    #[serde(rename = "OriginationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// <p>The sender ID to display as the sender of the message on a recipient's device. Support for sender IDs varies by country or region.</p>
    #[serde(rename = "SenderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// <p>The message variables to use in the SMS message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>Specifies the content and settings for a message template that can be used in text messages that are sent through the SMS channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SMSTemplateRequest {
    /// <p>The message body to use in text messages that are based on the message template.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>A JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>A custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about the content and settings for a message template that can be used in text messages that are sent through the SMS channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SMSTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the message template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The message body that's used in text messages that are based on the message template.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The JSON object that specifies the default values that are used for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. For an SMS template, this value is SMS.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The unique identifier, as an integer, for the active version of the message template, or the version of the template that you specified by using the version parameter in your request.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the schedule settings for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// <p>The scheduled time, in ISO 8601 format, when the campaign ended or will end.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>The type of event that causes the campaign to be sent, if the value of the Frequency property is EVENT.</p>
    #[serde(rename = "EventFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<CampaignEventFilter>,
    /// <p>Specifies how often the campaign is sent or whether the campaign is sent in response to a specific event.</p>
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// <p>Specifies whether the start and end times for the campaign schedule use each recipient's local time. To base the schedule on each recipient's local time, set this value to true.</p>
    #[serde(rename = "IsLocalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_time: Option<bool>,
    /// <p>The default quiet time for the campaign. Quiet time is a specific time range when a campaign doesn't send messages to endpoints, if all the following conditions are met:</p> <ul><li><p>The EndpointDemographic.Timezone property of the endpoint is set to a valid value.</p></li> <li><p>The current time in the endpoint's time zone is later than or equal to the time specified by the QuietTime.Start property for the campaign.</p></li> <li><p>The current time in the endpoint's time zone is earlier than or equal to the time specified by the QuietTime.End property for the campaign.</p></li></ul> <p>If any of the preceding conditions isn't met, the endpoint will receive messages from the campaign, even if quiet time is enabled.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    /// <p>The scheduled time, in ISO 8601 format, when the campaign began or will begin.</p>
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// <p>The starting UTC offset for the campaign schedule, if the value of the IsLocalTime property is true. Valid values are: UTC, UTC+01, UTC+02, UTC+03, UTC+03:30, UTC+04, UTC+04:30, UTC+05,
    /// UTC+05:30, UTC+05:45, UTC+06, UTC+06:30, UTC+07, UTC+08, UTC+09, UTC+09:30,
    /// UTC+10, UTC+10:30, UTC+11, UTC+12, UTC+13, UTC-02, UTC-03, UTC-04, UTC-05, UTC-06,
    /// UTC-07, UTC-08, UTC-09, UTC-10, and UTC-11.</p>
    #[serde(rename = "Timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>Specifies dimension settings for including or excluding endpoints from a segment based on how recently an endpoint was active.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentBehaviors {
    /// <p>The dimension settings that are based on how recently an endpoint was active.</p>
    #[serde(rename = "Recency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency: Option<RecencyDimension>,
}

/// <p>Specifies a segment to associate with an activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentCondition {
    /// <p>The unique identifier for the segment to associate with the activity.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
}

/// <p>Specifies demographic-based dimension settings for including or excluding endpoints from a segment. These settings derive from characteristics of endpoint devices, such as platform, make, and model.</p>
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

/// <p>Specifies the dimension settings for a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentDimensions {
    /// <p>One or more custom attributes to use as criteria for the segment.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
    /// <p>The behavior-based criteria, such as how recently users have used your app, for the segment.</p>
    #[serde(rename = "Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<SegmentBehaviors>,
    /// <p>The demographic-based criteria, such as device platform, for the segment.</p>
    #[serde(rename = "Demographic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<SegmentDemographics>,
    /// <p>The location-based criteria, such as region or GPS coordinates, for the segment.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SegmentLocation>,
    /// <p>One or more custom metrics to use as criteria for the segment.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, MetricDimension>>,
    /// <p>One or more custom user attributes to use as criteria for the segment.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::std::collections::HashMap<String, AttributeDimension>>,
}

/// <p>Specifies the base segments and dimensions for a segment, and the relationships between these base segments and dimensions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroup {
    /// <p>An array that defines the dimensions for the segment.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<SegmentDimensions>>,
    /// <p>The base segment to build the segment on. A base segment, also referred to as a <i>source segment</i>, defines the initial population of endpoints for a segment. When you add dimensions to a segment, Amazon Pinpoint filters the base segment by using the dimensions that you specify.</p> <p>You can specify more than one dimensional segment or only one imported segment. If you specify an imported segment, the Amazon Pinpoint console displays a segment size estimate that indicates the size of the imported segment without any filters applied to it.</p>
    #[serde(rename = "SourceSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_segments: Option<Vec<SegmentReference>>,
    /// <p>Specifies how to handle multiple base segments for the segment. For example, if you specify three base segments for the segment, whether the resulting segment is based on all, any, or none of the base segments.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>Specifies how to handle multiple dimensions for the segment. For example, if you specify three dimensions for the segment, whether the resulting segment includes endpoints that match all, any, or none of the dimensions.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Specifies the settings that define the relationships between segment groups for a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentGroupList {
    /// <p>An array that defines the set of segment criteria to evaluate when handling segment groups for the segment.</p>
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
    /// <p>Specifies how to handle multiple segment groups for the segment. For example, if the segment includes three segment groups, whether the resulting segment includes endpoints that match all, any, or none of the segment groups.</p>
    #[serde(rename = "Include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

/// <p>Provides information about the import job that created a segment. An import job is a job that creates a user segment by importing endpoint definitions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SegmentImportResource {
    /// <p>The number of channel types in the endpoint definitions that were imported to create the segment.</p>
    #[serde(rename = "ChannelCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_counts: Option<::std::collections::HashMap<String, i64>>,
    /// <p>(Deprecated) Your AWS account ID, which you assigned to an external ID key in an IAM trust policy. Amazon Pinpoint previously used this value to assume an IAM role when importing endpoint definitions, but we removed this requirement. We don't recommend use of external IDs for IAM roles that are assumed by Amazon Pinpoint.</p>
    #[serde(rename = "ExternalId")]
    pub external_id: String,
    /// <p>The format of the files that were imported to create the segment. Valid values are: CSV, for comma-separated values format; and, JSON, for newline-delimited JSON format.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that authorized Amazon Pinpoint to access the Amazon S3 location to import endpoint definitions from.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The URL of the Amazon Simple Storage Service (Amazon S3) bucket that the endpoint definitions were imported from to create the segment.</p>
    #[serde(rename = "S3Url")]
    pub s3_url: String,
    /// <p>The number of endpoint definitions that were imported successfully to create the segment.</p>
    #[serde(rename = "Size")]
    pub size: i64,
}

/// <p>Specifies geographical dimension settings for a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentLocation {
    /// <p>The country or region code, in ISO 3166-1 alpha-2 format, for the segment.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<SetDimension>,
    /// <p>The GPS location and range for the segment.</p>
    #[serde(rename = "GPSPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_point: Option<GPSPointDimension>,
}

/// <p>Specifies the segment identifier and version of a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentReference {
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The version number of the segment.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Provides information about the configuration, dimension, and other settings for a segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SegmentResponse {
    /// <p>The unique identifier for the application that the segment is associated with.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The Amazon Resource Name (ARN) of the segment.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The date and time when the segment was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The dimension settings for the segment.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The settings for the import job that's associated with the segment.</p>
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
    /// <p>A list of one or more segment groups that apply to the segment. Each segment group consists of zero or more base segments and the dimensions that are applied to those base segments.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    /// <p><p>The segment type. Valid values are:</p> <ul><li><p>DIMENSIONAL - A dynamic segment, which is a segment that uses selection criteria that you specify and is based on endpoint data that&#39;s reported by your app. Dynamic segments can change over time.</p></li> <li><p>IMPORT - A static segment, which is a segment that uses selection criteria that you specify and is based on endpoint definitions that you import from a file. Imported segments are static; they don&#39;t change over time.</p></li></ul></p>
    #[serde(rename = "SegmentType")]
    pub segment_type: String,
    /// <p>The version number of the segment.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the segment. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about all the segments that are associated with an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SegmentsResponse {
    /// <p>An array of responses, one for each segment that's associated with the application (Segments resource) or each version of a segment that's associated with the application (Segment Versions resource).</p>
    #[serde(rename = "Item")]
    pub item: Vec<SegmentResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendMessagesRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "MessageRequest")]
    pub message_request: MessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendMessagesResponse {
    #[serde(rename = "MessageResponse")]
    pub message_response: MessageResponse,
}

/// <p>Specifies the configuration and other settings for a message to send to all the endpoints that are associated with a list of users.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendUsersMessageRequest {
    /// <p>A map of custom attribute-value pairs. For a push notification, Amazon Pinpoint adds these attributes to the data.pinpoint object in the body of the notification payload. Amazon Pinpoint also provides these attributes in the events that it generates for users-messages deliveries.</p>
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The settings and content for the default message and any default messages that you defined for specific channels.</p>
    #[serde(rename = "MessageConfiguration")]
    pub message_configuration: DirectMessageConfiguration,
    /// <p>The message template to use for the message.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>The unique identifier for tracing the message. This identifier is visible to message recipients.</p>
    #[serde(rename = "TraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// <p>A map that associates user IDs with EndpointSendConfiguration objects. You can use an EndpointSendConfiguration object to tailor the message for a user by specifying settings such as content overrides and message variables.</p>
    #[serde(rename = "Users")]
    pub users: ::std::collections::HashMap<String, EndpointSendConfiguration>,
}

/// <p>Provides information about which users and endpoints a message was sent to.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendUsersMessageResponse {
    /// <p>The unique identifier for the application that was used to send the message.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier that was assigned to the message request.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>An object that indicates which endpoints the message was sent to, for each user. The object lists user IDs and, for each user ID, provides the endpoint IDs that the message was sent to. For each endpoint ID, it provides an EndpointMessageResult object.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendUsersMessagesRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SendUsersMessageRequest")]
    pub send_users_message_request: SendUsersMessageRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendUsersMessagesResponse {
    #[serde(rename = "SendUsersMessageResponse")]
    pub send_users_message_response: SendUsersMessageResponse,
}

/// <p>Provides information about a session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Session {
    /// <p>The duration of the session, in milliseconds.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The unique identifier for the session.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The date and time when the session began.</p>
    #[serde(rename = "StartTimestamp")]
    pub start_timestamp: String,
    /// <p>The date and time when the session ended.</p>
    #[serde(rename = "StopTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timestamp: Option<String>,
}

/// <p>Specifies the dimension type and values for a segment dimension.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDimension {
    /// <p>The type of segment dimension to use. Valid values are: INCLUSIVE, endpoints that match the criteria are included in the segment; and, EXCLUSIVE, endpoints that match the criteria are excluded from the segment.</p>
    #[serde(rename = "DimensionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    /// <p>The criteria values to use for the segment dimension. Depending on the value of the DimensionType property, endpoints are included or excluded from the segment if their values match the criteria values.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies a condition to evaluate for an activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleCondition {
    /// <p>The dimension settings for the event that's associated with the activity.</p>
    #[serde(rename = "EventCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_condition: Option<EventCondition>,
    /// <p>The segment that's associated with the activity.</p>
    #[serde(rename = "SegmentCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_condition: Option<SegmentCondition>,
    /// <p>The dimension settings for the segment that's associated with the activity.</p>
    #[serde(rename = "SegmentDimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_dimensions: Option<SegmentDimensions>,
}

/// <p>Specifies the contents of an email message, composed of a subject, a text part, and an HTML part.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SimpleEmail {
    /// <p>The body of the email message, in HTML format. We recommend using HTML format for email clients that render HTML content. You can include links, formatted text, and more in an HTML message.</p>
    #[serde(rename = "HtmlPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<SimpleEmailPart>,
    /// <p>The subject line, or title, of the email.</p>
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SimpleEmailPart>,
    /// <p>The body of the email message, in plain text format. We recommend using plain text format for email clients that don't render HTML content and clients that are connected to high-latency networks, such as mobile devices.</p>
    #[serde(rename = "TextPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<SimpleEmailPart>,
}

/// <p>Specifies the subject or body of an email message, represented as textual email data and the applicable character set.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SimpleEmailPart {
    /// <p>The applicable character set for the message content.</p>
    #[serde(rename = "Charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// <p>The textual data of the message content.</p>
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// <p>Specifies the conditions for the first activity in a journey. This activity and its conditions determine which users are participants in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartCondition {
    /// <p>The custom description of the condition.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The segment that's associated with the first activity in the journey. This segment determines which users are participants in the journey.</p>
    #[serde(rename = "SegmentStartCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_start_condition: Option<SegmentCondition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "TagsModel")]
    pub tags_model: TagsModel,
}

/// <p>Specifies the tags (keys and values) for an application, campaign, journey, message template, or segment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagsModel {
    /// <p>A string-to-string map of key-value pairs that defines the tags for an application, campaign, journey, message template, or segment. Each of these resources can have a maximum of 50 tags.</p> <p>Each tag consists of a required tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>Specifies the name and version of the message template to use for the message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    /// <p>The name of the message template to use for the message. If specified, this value must match the name of an existing message template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier for the version of the message template to use for the message. If specified, this value must match the identifier for an existing template version. To retrieve a list of versions and version identifiers for a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If you don't specify a value for this property, Amazon Pinpoint uses the <i>active</i> version of the template. The <i>active</i> version is typically the version of a template that's been most recently reviewed and approved for use, depending on your workflow. It isn't necessarily the latest version of a template.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Specifies which version of a message template to use as the active version of the template.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TemplateActiveVersionRequest {
    /// <p>The unique identifier for the version of the message template to use as the active version of the template. If specified, this value must match the identifier for an existing template version. To retrieve a list of versions and version identifiers for a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Specifies the message template to use for the message, for each type of channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateConfiguration {
    /// <p>The email template to use for the message.</p>
    #[serde(rename = "EmailTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_template: Option<Template>,
    /// <p>The push notification template to use for the message.</p>
    #[serde(rename = "PushTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_template: Option<Template>,
    /// <p>The SMS template to use for the message.</p>
    #[serde(rename = "SMSTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_template: Option<Template>,
    /// <p>The voice template to use for the message.</p>
    #[serde(rename = "VoiceTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_template: Option<Template>,
}

/// <p>Provides information about a message template that's associated with your Amazon Pinpoint account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the message template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The JSON object that specifies the default values that are used for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. Possible values are: EMAIL, PUSH, SMS, and VOICE.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The unique identifier, as an integer, for the active version of the message template.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about a specific version of a message template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateVersionResponse {
    /// <p>The date, in ISO 8601 format, when the version of the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>A JSON object that specifies the default values that are used for message variables in the version of the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The date, in ISO 8601 format, when the version of the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The custom description of the version of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. Possible values are: EMAIL, PUSH, SMS, and VOICE.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The unique identifier for the version of the message template. This value is an integer that Amazon Pinpoint automatically increments and assigns to each new version of a template.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Provides information about all the versions of a specific message template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplateVersionsResponse {
    /// <p>An array of responses, one for each version of the message template.</p>
    #[serde(rename = "Item")]
    pub item: Vec<TemplateVersionResponse>,
    /// <p>The message that's returned from the API for the request to retrieve information about all the versions of the message template.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier for the request to retrieve information about all the versions of the message template.</p>
    #[serde(rename = "RequestID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// <p>Provides information about all the message templates that are associated with your Amazon Pinpoint account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TemplatesResponse {
    /// <p>An array of responses, one for each message template that's associated with your Amazon Pinpoint account and meets any filter criteria that you specified in the request.</p>
    #[serde(rename = "Item")]
    pub item: Vec<TemplateResponse>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Specifies the settings for a campaign treatment. A treatment is a variation of a campaign that's used for A/B testing of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TreatmentResource {
    /// <p>The unique identifier for the treatment.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The message configuration settings for the treatment.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The schedule settings for the treatment.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users (segment members) that the treatment is sent to.</p>
    #[serde(rename = "SizePercent")]
    pub size_percent: i64,
    /// <p>The current status of the treatment.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    /// <p>The message template to use for the treatment.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>The custom description of the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of the treatment. A treatment is a variation of a campaign that's used for A/B testing of a campaign.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key of the tag to remove from the resource. To remove multiple tags, append the tagKeys parameter and argument for each additional tag to remove, separated by an ampersand (&amp;).</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAdmChannelRequest {
    #[serde(rename = "ADMChannelRequest")]
    pub adm_channel_request: ADMChannelRequest,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    pub adm_channel_response: ADMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApnsChannelRequest {
    #[serde(rename = "APNSChannelRequest")]
    pub apns_channel_request: APNSChannelRequest,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    pub apns_channel_response: APNSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApnsSandboxChannelRequest {
    #[serde(rename = "APNSSandboxChannelRequest")]
    pub apns_sandbox_channel_request: APNSSandboxChannelRequest,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    pub apns_sandbox_channel_response: APNSSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApnsVoipChannelRequest {
    #[serde(rename = "APNSVoipChannelRequest")]
    pub apns_voip_channel_request: APNSVoipChannelRequest,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    pub apns_voip_channel_response: APNSVoipChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApnsVoipSandboxChannelRequest {
    #[serde(rename = "APNSVoipSandboxChannelRequest")]
    pub apns_voip_sandbox_channel_request: APNSVoipSandboxChannelRequest,
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    pub apns_voip_sandbox_channel_response: APNSVoipSandboxChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationSettingsRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "WriteApplicationSettingsRequest")]
    pub write_application_settings_request: WriteApplicationSettingsRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    pub application_settings_resource: ApplicationSettingsResource,
}

/// <p>Specifies one or more attributes to remove from all the endpoints that are associated with an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAttributesRequest {
    /// <p>An array of the attributes to remove from all the endpoints that are associated with the application. The array can specify the complete, exact name of each attribute to remove or it can specify a glob pattern that an attribute name must match in order for the attribute to be removed.</p>
    #[serde(rename = "Blacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBaiduChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "BaiduChannelRequest")]
    pub baidu_channel_request: BaiduChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    pub baidu_channel_response: BaiduChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCampaignRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the campaign.</p>
    #[serde(rename = "CampaignId")]
    pub campaign_id: String,
    #[serde(rename = "WriteCampaignRequest")]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    pub campaign_response: CampaignResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmailChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EmailChannelRequest")]
    pub email_channel_request: EmailChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    pub email_channel_response: EmailChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEmailTemplateRequest {
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to the latest existing version of the template.</p><p> If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    #[serde(rename = "CreateNewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_new_version: Option<bool>,
    #[serde(rename = "EmailTemplateRequest")]
    pub email_template_request: EmailTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEmailTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the endpoint.</p>
    #[serde(rename = "EndpointId")]
    pub endpoint_id: String,
    #[serde(rename = "EndpointRequest")]
    pub endpoint_request: EndpointRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointsBatchRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "EndpointBatchRequest")]
    pub endpoint_batch_request: EndpointBatchRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointsBatchResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGcmChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "GCMChannelRequest")]
    pub gcm_channel_request: GCMChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    pub gcm_channel_response: GCMChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJourneyRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    #[serde(rename = "WriteJourneyRequest")]
    pub write_journey_request: WriteJourneyRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    pub journey_response: JourneyResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJourneyStateRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the journey.</p>
    #[serde(rename = "JourneyId")]
    pub journey_id: String,
    #[serde(rename = "JourneyStateRequest")]
    pub journey_state_request: JourneyStateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJourneyStateResponse {
    #[serde(rename = "JourneyResponse")]
    pub journey_response: JourneyResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePushTemplateRequest {
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to the latest existing version of the template.</p><p> If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    #[serde(rename = "CreateNewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_new_version: Option<bool>,
    #[serde(rename = "PushNotificationTemplateRequest")]
    pub push_notification_template_request: PushNotificationTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePushTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSegmentRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The unique identifier for the segment.</p>
    #[serde(rename = "SegmentId")]
    pub segment_id: String,
    #[serde(rename = "WriteSegmentRequest")]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    pub segment_response: SegmentResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSmsChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "SMSChannelRequest")]
    pub sms_channel_request: SMSChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    pub sms_channel_response: SMSChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSmsTemplateRequest {
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to the latest existing version of the template.</p><p> If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    #[serde(rename = "CreateNewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_new_version: Option<bool>,
    #[serde(rename = "SMSTemplateRequest")]
    pub sms_template_request: SMSTemplateRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSmsTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTemplateActiveVersionRequest {
    #[serde(rename = "TemplateActiveVersionRequest")]
    pub template_active_version_request: TemplateActiveVersionRequest,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. Valid values are: EMAIL, PUSH, SMS, and VOICE.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTemplateActiveVersionResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVoiceChannelRequest {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    #[serde(rename = "VoiceChannelRequest")]
    pub voice_channel_request: VoiceChannelRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    pub voice_channel_response: VoiceChannelResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVoiceTemplateRequest {
    /// <p>Specifies whether to save the updates as a new version of the message template. Valid values are: true, save the updates as a new version; and, false, save the updates to the latest existing version of the template.</p><p> If you don't specify a value for this parameter, Amazon Pinpoint saves the updates to the latest existing version of the template. If you specify a value of true for this parameter, don't specify a value for the version parameter. Otherwise, an error will occur.</p>
    #[serde(rename = "CreateNewVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_new_version: Option<bool>,
    /// <p>The name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p><p>The unique identifier for the version of the message template to update, retrieve information about, or delete. To retrieve identifiers and other information for all the versions of a template, use the <link  linkend="templates-template-name-template-type-versions">Template Versions</link> resource.</p> <p>If specified, this value must match the identifier of an existing template version. If specified for an update operation, this value must match the identifier of the latest existing version of the template. This restriction helps ensure that race conditions don&#39;t occur.</p> <p>If you don&#39;t specify a value for this parameter, Amazon Pinpoint does the following:</p> <ul><li><p>For a get operation, retrieves information about the active version of the template.</p></li> <li><p>For an update operation, saves the updates to the latest existing version of the template, if the create-new-version parameter isn&#39;t used or is set to false.</p></li> <li><p>For a delete operation, deletes the template, including all versions of the template.</p></li></ul></p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VoiceTemplateRequest")]
    pub voice_template_request: VoiceTemplateRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVoiceTemplateResponse {
    #[serde(rename = "MessageBody")]
    pub message_body: MessageBody,
}

/// <p>Specifies the status and settings of the voice channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VoiceChannelRequest {
    /// <p>Specifies whether to enable the voice channel for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// <p>Provides information about the status and settings of the voice channel for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoiceChannelResponse {
    /// <p>The unique identifier for the application that the voice channel applies to.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the voice channel was enabled.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>Specifies whether the voice channel is enabled for the application.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>(Not used) This property is retained only for backward compatibility.</p>
    #[serde(rename = "HasCredential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    /// <p>(Deprecated) An identifier for the voice channel. This property is retained only for backward compatibility.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Specifies whether the voice channel is archived.</p>
    #[serde(rename = "IsArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// <p>The user who last modified the voice channel.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when the voice channel was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The type of messaging or notification platform for the channel. For the voice channel, this value is VOICE.</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The current version of the voice channel.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Specifies the settings for a one-time voice message that's sent directly to an endpoint through the voice channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VoiceMessage {
    /// <p>The text of the script to use for the voice message.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The code for the language to use when synthesizing the text of the message script. For a list of supported languages and the code for each one, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The long code to send the voice message from. This value should be one of the dedicated long codes that's assigned to your AWS account. Although it isn't required, we recommend that you specify the long code in E.164 format, for example +12065550100, to ensure prompt and accurate delivery of the message.</p>
    #[serde(rename = "OriginationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// <p>The default message variables to use in the voice message. You can override the default variables with individual address variables.</p>
    #[serde(rename = "Substitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The name of the voice to use when delivering the message. For a list of supported voices, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

/// <p>Specifies the content and settings for a message template that can be used in messages that are sent through the voice channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VoiceTemplateRequest {
    /// <p>The text of the script to use in messages that are based on the message template, in plain text format.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>A JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The code for the language to use when synthesizing the text of the script in messages that are based on the message template. For a list of supported languages and the code for each one, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>A custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the voice to use when delivering messages that are based on the message template. For a list of supported voices, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides information about the content and settings for a message template that can be used in messages that are sent through the voice channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VoiceTemplateResponse {
    /// <p>The Amazon Resource Name (ARN) of the message template.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The text of the script that's used in messages that are based on the message template, in plain text format.</p>
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was created.</p>
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
    /// <p>The JSON object that specifies the default values that are used for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable.</p>
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    /// <p>The code for the language that's used when synthesizing the text of the script in messages that are based on the message template. For a list of supported languages and the code for each one, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date, in ISO 8601 format, when the message template was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: String,
    /// <p>The custom description of the message template.</p>
    #[serde(rename = "TemplateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    /// <p>The name of the message template.</p>
    #[serde(rename = "TemplateName")]
    pub template_name: String,
    /// <p>The type of channel that the message template is designed for. For a voice template, this value is VOICE.</p>
    #[serde(rename = "TemplateType")]
    pub template_type: String,
    /// <p>The unique identifier, as an integer, for the active version of the message template, or the version of the template that you specified by using the version parameter in your request.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The name of the voice that's used when delivering messages that are based on the message template. For a list of supported voices, see the <a href="https://docs.aws.amazon.com/polly/latest/dg/what-is.html">Amazon Polly Developer Guide</a>.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// <p>A string-to-string map of key-value pairs that identifies the tags that are associated with the message template. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the settings for a wait activity in a journey. This type of activity waits for a certain amount of time or until a specific date and time before moving participants to the next activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitActivity {
    /// <p>The unique identifier for the next activity to perform, after performing the wait activity.</p>
    #[serde(rename = "NextActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    /// <p>The amount of time to wait or the date and time when the activity moves participants to the next activity in the journey.</p>
    #[serde(rename = "WaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<WaitTime>,
}

/// <p>Specifies a duration or a date and time that indicates when Amazon Pinpoint determines whether an activity's conditions have been met or an activity moves participants to the next activity in a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitTime {
    /// <p>The amount of time to wait, as a duration in ISO 8601 format, before determining whether the activity's conditions have been met or moving participants to the next activity in the journey.</p>
    #[serde(rename = "WaitFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    /// <p>The date and time, in ISO 8601 format, when Amazon Pinpoint determines whether the activity's conditions have been met or the activity moves participants to the next activity in the journey.</p>
    #[serde(rename = "WaitUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<String>,
}

/// <p>Specifies the default settings for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteApplicationSettingsRequest {
    /// <p>The settings for the AWS Lambda function to use by default as a code hook for campaigns in the application. To override these settings for a specific campaign, use the <link  linkend="apps-application-id-campaigns-campaign-id">Campaign</link> resource to define custom Lambda function settings for the campaign.</p>
    #[serde(rename = "CampaignHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    /// <p>Specifies whether to enable application-related alarms in Amazon CloudWatch.</p>
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    /// <p>The default sending limits for campaigns in the application. To override these limits for a specific campaign, use the <link  linkend="apps-application-id-campaigns-campaign-id">Campaign</link> resource to define custom limits for the campaign.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The default quiet time for campaigns and journeys in the application. Quiet time is a specific time range when messages aren't sent to endpoints, if all the following conditions are met:</p> <ul><li><p>The EndpointDemographic.Timezone property of the endpoint is set to a valid value.</p></li> <li><p>The current time in the endpoint's time zone is later than or equal to the time specified by the QuietTime.Start property for the application (or a campaign or journey that has custom quiet time settings).</p></li> <li><p>The current time in the endpoint's time zone is earlier than or equal to the time specified by the QuietTime.End property for the application (or a campaign or journey that has custom quiet time settings).</p></li></ul> <p>If any of the preceding conditions isn't met, the endpoint will receive messages from a campaign or journey, even if quiet time is enabled.</p> <p>To override the default quiet time settings for a specific campaign or journey, use the <link  linkend="apps-application-id-campaigns-campaign-id">Campaign</link> resource or the <link  linkend="apps-application-id-journeys-journey-id">Journey</link> resource to define a custom quiet time for the campaign or journey.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

/// <p>Specifies the configuration and other settings for a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteCampaignRequest {
    /// <p>An array of requests that defines additional treatments for the campaign, in addition to the default treatment for the campaign.</p>
    #[serde(rename = "AdditionalTreatments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<WriteTreatmentResource>>,
    /// <p>A custom description of the campaign.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The allocated percentage of users (segment members) who shouldn't receive messages from the campaign.</p>
    #[serde(rename = "HoldoutPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i64>,
    /// <p>The settings for the AWS Lambda function to use as a code hook for the campaign.</p>
    #[serde(rename = "Hook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    /// <p>Specifies whether to pause the campaign. A paused campaign doesn't run unless you resume it by setting this value to false.</p>
    #[serde(rename = "IsPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// <p>The messaging limits for the campaign.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    /// <p>The message configuration settings for the campaign.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The custom name of the campaign.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schedule settings for the campaign.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The unique identifier for the segment to associate with the campaign.</p>
    #[serde(rename = "SegmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// <p>The version of the segment to associate with the campaign.</p>
    #[serde(rename = "SegmentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i64>,
    /// <p>The message template to use for the campaign.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>A custom description of a variation of the campaign to use for A/B testing.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of a variation of the campaign to use for A/B testing.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the campaign. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the Amazon Resource Name (ARN) of an event stream to publish events to and the AWS Identity and Access Management (IAM) role to use when publishing those events.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteEventStream {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis data stream or Amazon Kinesis Data Firehose delivery stream that you want to publish event data to.</p> <p>For a Kinesis data stream, the ARN format is: arn:aws:kinesis:<replaceable>region</replaceable>:<replaceable>account-id</replaceable>:stream/<replaceable>stream_name</replaceable>
    /// </p> <p>For a Kinesis Data Firehose delivery stream, the ARN format is: arn:aws:firehose:<replaceable>region</replaceable>:<replaceable>account-id</replaceable>:deliverystream/<replaceable>stream_name</replaceable>
    /// </p>
    #[serde(rename = "DestinationStreamArn")]
    pub destination_stream_arn: String,
    /// <p>The AWS Identity and Access Management (IAM) role that authorizes Amazon Pinpoint to publish event data to the stream in your AWS account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>Specifies the configuration and other settings for a journey.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteJourneyRequest {
    /// <p>A map that contains a set of Activity objects, one object for each activity in the journey. For each Activity object, the key is the unique identifier (string) for an activity and the value is the settings for the activity. An activity identifier can contain a maximum of 128 characters. The characters must be alphanumeric characters.</p>
    #[serde(rename = "Activities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<::std::collections::HashMap<String, Activity>>,
    /// <p>The date, in ISO 8601 format, when the journey was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// <p>The date, in ISO 8601 format, when the journey was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The messaging and entry limits for the journey.</p>
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<JourneyLimits>,
    /// <p>Specifies whether the journey's scheduled start and end times use each participant's local time. To base the schedule on each participant's local time, set this value to true.</p>
    #[serde(rename = "LocalTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<bool>,
    /// <p>The name of the journey. A journey name can contain a maximum of 150 characters. The characters can be alphanumeric characters or symbols, such as underscores (_) or hyphens (-). A journey name can't contain any spaces.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The quiet time settings for the journey. Quiet time is a specific time range when a journey doesn't send messages to participants, if all the following conditions are met:</p> <ul><li><p>The EndpointDemographic.Timezone property of the endpoint for the participant is set to a valid value.</p></li> <li><p>The current time in the participant's time zone is later than or equal to the time specified by the QuietTime.Start property for the journey.</p></li> <li><p>The current time in the participant's time zone is earlier than or equal to the time specified by the QuietTime.End property for the journey.</p></li></ul> <p>If any of the preceding conditions isn't met, the participant will receive messages from the journey, even if quiet time is enabled.</p>
    #[serde(rename = "QuietTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    /// <p>The frequency with which Amazon Pinpoint evaluates segment and event data for the journey, as a duration in ISO 8601 format.</p>
    #[serde(rename = "RefreshFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_frequency: Option<String>,
    /// <p>The schedule settings for the journey.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JourneySchedule>,
    /// <p>The unique identifier for the first activity in the journey. An activity identifier can contain a maximum of 128 characters. The characters must be alphanumeric characters.</p>
    #[serde(rename = "StartActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_activity: Option<String>,
    /// <p>The segment that defines which users are participants in the journey.</p>
    #[serde(rename = "StartCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_condition: Option<StartCondition>,
    /// <p>The status of the journey. Valid values are:</p> <ul><li><p>DRAFT - Saves the journey and doesn't publish it.</p></li> <li><p>ACTIVE - Saves and publishes the journey. Depending on the journey's schedule, the journey starts running immediately or at the scheduled start time. If a journey's status is ACTIVE, you can't add, change, or remove activities from it.</p></li></ul> <p>The CANCELLED, COMPLETED, and CLOSED values are not supported in requests to create or update a journey. To cancel a journey, use the <link  linkend="apps-application-id-journeys-journey-id-state">Journey State</link> resource.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Specifies the configuration, dimension, and other settings for a segment. A WriteSegmentRequest object can include a Dimensions object or a SegmentGroups object, but not both.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteSegmentRequest {
    /// <p>The criteria that define the dimensions for the segment.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    /// <p>The name of the segment.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The segment group to use and the dimensions to apply to the group's base segments in order to build the segment. A segment group can consist of zero or more base segments. Your request can include only one segment group.</p>
    #[serde(rename = "SegmentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    /// <p>A string-to-string map of key-value pairs that defines the tags to associate with the segment. Each tag consists of a required tag key and an associated tag value.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies the settings for a campaign treatment. A treatment is a variation of a campaign that's used for A/B testing of a campaign.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteTreatmentResource {
    /// <p>The message configuration settings for the treatment.</p>
    #[serde(rename = "MessageConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    /// <p>The schedule settings for the treatment.</p>
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    /// <p>The allocated percentage of users (segment members) to send the treatment to.</p>
    #[serde(rename = "SizePercent")]
    pub size_percent: i64,
    /// <p>The message template to use for the treatment.</p>
    #[serde(rename = "TemplateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    /// <p>A custom description of the treatment.</p>
    #[serde(rename = "TreatmentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    /// <p>The custom name of the treatment. A treatment is a variation of a campaign that's used for A/B testing of a campaign.</p>
    #[serde(rename = "TreatmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateAppError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateAppError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateAppError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateAppError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateAppError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAppError {}
/// Errors returned by CreateCampaign
#[derive(Debug, PartialEq)]
pub enum CreateCampaignError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCampaignError {}
/// Errors returned by CreateEmailTemplate
#[derive(Debug, PartialEq)]
pub enum CreateEmailTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl CreateEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateEmailTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateEmailTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateEmailTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateEmailTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEmailTemplateError {}
/// Errors returned by CreateExportJob
#[derive(Debug, PartialEq)]
pub enum CreateExportJobError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateExportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateExportJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateExportJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateExportJobError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateExportJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateExportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateExportJobError {}
/// Errors returned by CreateImportJob
#[derive(Debug, PartialEq)]
pub enum CreateImportJobError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateImportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateImportJobError {}
/// Errors returned by CreateJourney
#[derive(Debug, PartialEq)]
pub enum CreateJourneyError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl CreateJourneyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJourneyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateJourneyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateJourneyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateJourneyError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateJourneyError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateJourneyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateJourneyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJourneyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJourneyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateJourneyError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateJourneyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateJourneyError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateJourneyError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateJourneyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJourneyError {}
/// Errors returned by CreatePushTemplate
#[derive(Debug, PartialEq)]
pub enum CreatePushTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl CreatePushTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePushTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreatePushTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePushTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreatePushTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreatePushTemplateError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePushTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePushTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePushTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePushTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePushTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreatePushTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreatePushTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePushTemplateError {}
/// Errors returned by CreateSegment
#[derive(Debug, PartialEq)]
pub enum CreateSegmentError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSegmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSegmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateSegmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateSegmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateSegmentError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateSegmentError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateSegmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSegmentError {}
/// Errors returned by CreateSmsTemplate
#[derive(Debug, PartialEq)]
pub enum CreateSmsTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl CreateSmsTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSmsTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateSmsTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateSmsTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateSmsTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateSmsTemplateError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateSmsTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSmsTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSmsTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateSmsTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateSmsTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateSmsTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateSmsTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSmsTemplateError {}
/// Errors returned by CreateVoiceTemplate
#[derive(Debug, PartialEq)]
pub enum CreateVoiceTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl CreateVoiceTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVoiceTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVoiceTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateVoiceTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateVoiceTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(CreateVoiceTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVoiceTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVoiceTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVoiceTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVoiceTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateVoiceTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateVoiceTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            CreateVoiceTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVoiceTemplateError {}
/// Errors returned by DeleteAdmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteAdmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAdmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAdmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAdmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteAdmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteAdmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteAdmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAdmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAdmChannelError {}
/// Errors returned by DeleteApnsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApnsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApnsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApnsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteApnsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteApnsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteApnsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApnsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApnsChannelError {}
/// Errors returned by DeleteApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApnsSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApnsSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApnsSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteApnsSandboxChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteApnsSandboxChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteApnsSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApnsSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApnsSandboxChannelError {}
/// Errors returned by DeleteApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApnsVoipChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApnsVoipChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApnsVoipChannelError {}
/// Errors returned by DeleteApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum DeleteApnsVoipSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApnsVoipSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApnsVoipSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipSandboxChannelError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApnsVoipSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteApnsVoipSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApnsVoipSandboxChannelError {}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteAppError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteAppError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteAppError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteAppError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAppError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppError {}
/// Errors returned by DeleteBaiduChannel
#[derive(Debug, PartialEq)]
pub enum DeleteBaiduChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBaiduChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBaiduChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteBaiduChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteBaiduChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteBaiduChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteBaiduChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBaiduChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBaiduChannelError {}
/// Errors returned by DeleteCampaign
#[derive(Debug, PartialEq)]
pub enum DeleteCampaignError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCampaignError {}
/// Errors returned by DeleteEmailChannel
#[derive(Debug, PartialEq)]
pub enum DeleteEmailChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEmailChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEmailChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteEmailChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteEmailChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailChannelError {}
/// Errors returned by DeleteEmailTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteEmailTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl DeleteEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailTemplateError {}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEndpointError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEndpointError {}
/// Errors returned by DeleteEventStream
#[derive(Debug, PartialEq)]
pub enum DeleteEventStreamError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventStreamError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEventStreamError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteEventStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteEventStreamError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteEventStreamError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEventStreamError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventStreamError {}
/// Errors returned by DeleteGcmChannel
#[derive(Debug, PartialEq)]
pub enum DeleteGcmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGcmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGcmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteGcmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteGcmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteGcmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteGcmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteGcmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGcmChannelError {}
/// Errors returned by DeleteJourney
#[derive(Debug, PartialEq)]
pub enum DeleteJourneyError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl DeleteJourneyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJourneyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteJourneyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteJourneyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteJourneyError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteJourneyError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteJourneyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteJourneyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJourneyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJourneyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteJourneyError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteJourneyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteJourneyError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteJourneyError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteJourneyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJourneyError {}
/// Errors returned by DeletePushTemplate
#[derive(Debug, PartialEq)]
pub enum DeletePushTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl DeletePushTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePushTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeletePushTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePushTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeletePushTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeletePushTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePushTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePushTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePushTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePushTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeletePushTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePushTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeletePushTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeletePushTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePushTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePushTemplateError {}
/// Errors returned by DeleteSegment
#[derive(Debug, PartialEq)]
pub enum DeleteSegmentError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSegmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSegmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSegmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteSegmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSegmentError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteSegmentError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSegmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSegmentError {}
/// Errors returned by DeleteSmsChannel
#[derive(Debug, PartialEq)]
pub enum DeleteSmsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSmsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSmsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSmsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteSmsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSmsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteSmsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSmsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSmsChannelError {}
/// Errors returned by DeleteSmsTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteSmsTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl DeleteSmsTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSmsTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteSmsTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSmsTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSmsTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteSmsTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteSmsTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSmsTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteSmsTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteSmsTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSmsTemplateError {}
/// Errors returned by DeleteUserEndpoints
#[derive(Debug, PartialEq)]
pub enum DeleteUserEndpointsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteUserEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserEndpointsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteUserEndpointsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteUserEndpointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteUserEndpointsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteUserEndpointsError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteUserEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserEndpointsError {}
/// Errors returned by DeleteVoiceChannel
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVoiceChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteVoiceChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVoiceChannelError {}
/// Errors returned by DeleteVoiceTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteVoiceTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl DeleteVoiceTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVoiceTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVoiceTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVoiceTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVoiceTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVoiceTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteVoiceTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVoiceTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            DeleteVoiceTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVoiceTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVoiceTemplateError {}
/// Errors returned by GetAdmChannel
#[derive(Debug, PartialEq)]
pub enum GetAdmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAdmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAdmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAdmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAdmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetAdmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetAdmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAdmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAdmChannelError {}
/// Errors returned by GetApnsChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApnsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApnsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApnsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApnsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApnsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApnsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApnsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApnsChannelError {}
/// Errors returned by GetApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApnsSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApnsSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApnsSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApnsSandboxChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApnsSandboxChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApnsSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApnsSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApnsSandboxChannelError {}
/// Errors returned by GetApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApnsVoipChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApnsVoipChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApnsVoipChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApnsVoipChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApnsVoipChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApnsVoipChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApnsVoipChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApnsVoipChannelError {}
/// Errors returned by GetApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum GetApnsVoipSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApnsVoipSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApnsVoipSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApnsVoipSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApnsVoipSandboxChannelError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApnsVoipSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApnsVoipSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApnsVoipSandboxChannelError {}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAppError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAppError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetAppError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetAppError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAppError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppError {}
/// Errors returned by GetApplicationDateRangeKpi
#[derive(Debug, PartialEq)]
pub enum GetApplicationDateRangeKpiError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetApplicationDateRangeKpiError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetApplicationDateRangeKpiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApplicationDateRangeKpiError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetApplicationDateRangeKpiError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetApplicationDateRangeKpiError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetApplicationDateRangeKpiError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetApplicationDateRangeKpiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApplicationDateRangeKpiError::TooManyRequests(
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
impl fmt::Display for GetApplicationDateRangeKpiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationDateRangeKpiError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApplicationDateRangeKpiError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApplicationDateRangeKpiError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetApplicationDateRangeKpiError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApplicationDateRangeKpiError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApplicationDateRangeKpiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationDateRangeKpiError {}
/// Errors returned by GetApplicationSettings
#[derive(Debug, PartialEq)]
pub enum GetApplicationSettingsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetApplicationSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetApplicationSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetApplicationSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetApplicationSettingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetApplicationSettingsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetApplicationSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetApplicationSettingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetApplicationSettingsError {}
/// Errors returned by GetApps
#[derive(Debug, PartialEq)]
pub enum GetAppsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAppsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetAppsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetAppsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetAppsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetAppsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppsError {}
/// Errors returned by GetBaiduChannel
#[derive(Debug, PartialEq)]
pub enum GetBaiduChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBaiduChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBaiduChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBaiduChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetBaiduChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetBaiduChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetBaiduChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBaiduChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBaiduChannelError {}
/// Errors returned by GetCampaign
#[derive(Debug, PartialEq)]
pub enum GetCampaignError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignError {}
/// Errors returned by GetCampaignActivities
#[derive(Debug, PartialEq)]
pub enum GetCampaignActivitiesError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCampaignActivitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignActivitiesError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignActivitiesError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignActivitiesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignActivitiesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignActivitiesError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignActivitiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignActivitiesError {}
/// Errors returned by GetCampaignDateRangeKpi
#[derive(Debug, PartialEq)]
pub enum GetCampaignDateRangeKpiError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetCampaignDateRangeKpiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCampaignDateRangeKpiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCampaignDateRangeKpiError::TooManyRequests(
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
impl fmt::Display for GetCampaignDateRangeKpiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignDateRangeKpiError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignDateRangeKpiError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignDateRangeKpiError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignDateRangeKpiError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignDateRangeKpiError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignDateRangeKpiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignDateRangeKpiError {}
/// Errors returned by GetCampaignVersion
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCampaignVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignVersionError {}
/// Errors returned by GetCampaignVersions
#[derive(Debug, PartialEq)]
pub enum GetCampaignVersionsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCampaignVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignVersionsError {}
/// Errors returned by GetCampaigns
#[derive(Debug, PartialEq)]
pub enum GetCampaignsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCampaignsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCampaignsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetCampaignsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetCampaignsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCampaignsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetCampaignsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetCampaignsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCampaignsError {}
/// Errors returned by GetChannels
#[derive(Debug, PartialEq)]
pub enum GetChannelsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetChannelsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetChannelsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetChannelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetChannelsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetChannelsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetChannelsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetChannelsError {}
/// Errors returned by GetEmailChannel
#[derive(Debug, PartialEq)]
pub enum GetEmailChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEmailChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetEmailChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetEmailChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetEmailChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailChannelError {}
/// Errors returned by GetEmailTemplate
#[derive(Debug, PartialEq)]
pub enum GetEmailTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetEmailTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetEmailTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetEmailTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailTemplateError {}
/// Errors returned by GetEndpoint
#[derive(Debug, PartialEq)]
pub enum GetEndpointError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEndpointError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetEndpointError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEndpointError {}
/// Errors returned by GetEventStream
#[derive(Debug, PartialEq)]
pub enum GetEventStreamError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEventStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventStreamError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEventStreamError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetEventStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetEventStreamError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetEventStreamError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEventStreamError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventStreamError {}
/// Errors returned by GetExportJob
#[derive(Debug, PartialEq)]
pub enum GetExportJobError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetExportJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetExportJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetExportJobError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetExportJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetExportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExportJobError {}
/// Errors returned by GetExportJobs
#[derive(Debug, PartialEq)]
pub enum GetExportJobsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExportJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetExportJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetExportJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetExportJobsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetExportJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetExportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExportJobsError {}
/// Errors returned by GetGcmChannel
#[derive(Debug, PartialEq)]
pub enum GetGcmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGcmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGcmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetGcmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetGcmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetGcmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetGcmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetGcmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGcmChannelError {}
/// Errors returned by GetImportJob
#[derive(Debug, PartialEq)]
pub enum GetImportJobError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImportJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetImportJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImportJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetImportJobError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetImportJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetImportJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImportJobError {}
/// Errors returned by GetImportJobs
#[derive(Debug, PartialEq)]
pub enum GetImportJobsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetImportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetImportJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetImportJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetImportJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetImportJobsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetImportJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetImportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetImportJobsError {}
/// Errors returned by GetJourney
#[derive(Debug, PartialEq)]
pub enum GetJourneyError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetJourneyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJourneyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJourneyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetJourneyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJourneyError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetJourneyError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJourneyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJourneyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJourneyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJourneyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJourneyError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJourneyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJourneyError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetJourneyError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJourneyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJourneyError {}
/// Errors returned by GetJourneyDateRangeKpi
#[derive(Debug, PartialEq)]
pub enum GetJourneyDateRangeKpiError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetJourneyDateRangeKpiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJourneyDateRangeKpiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJourneyDateRangeKpiError::TooManyRequests(
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
impl fmt::Display for GetJourneyDateRangeKpiError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJourneyDateRangeKpiError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJourneyDateRangeKpiError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJourneyDateRangeKpiError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJourneyDateRangeKpiError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetJourneyDateRangeKpiError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJourneyDateRangeKpiError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJourneyDateRangeKpiError {}
/// Errors returned by GetJourneyExecutionActivityMetrics
#[derive(Debug, PartialEq)]
pub enum GetJourneyExecutionActivityMetricsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetJourneyExecutionActivityMetricsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetJourneyExecutionActivityMetricsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionActivityMetricsError::BadRequest(err.msg),
                    )
                }
                "ForbiddenException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionActivityMetricsError::Forbidden(err.msg),
                    )
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionActivityMetricsError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionActivityMetricsError::MethodNotAllowed(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJourneyExecutionActivityMetricsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionActivityMetricsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJourneyExecutionActivityMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJourneyExecutionActivityMetricsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetJourneyExecutionActivityMetricsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionActivityMetricsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetJourneyExecutionActivityMetricsError::MethodNotAllowed(ref cause) => {
                write!(f, "{}", cause)
            }
            GetJourneyExecutionActivityMetricsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionActivityMetricsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetJourneyExecutionActivityMetricsError {}
/// Errors returned by GetJourneyExecutionMetrics
#[derive(Debug, PartialEq)]
pub enum GetJourneyExecutionMetricsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetJourneyExecutionMetricsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetJourneyExecutionMetricsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJourneyExecutionMetricsError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetJourneyExecutionMetricsError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        GetJourneyExecutionMetricsError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetJourneyExecutionMetricsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJourneyExecutionMetricsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJourneyExecutionMetricsError::TooManyRequests(
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
impl fmt::Display for GetJourneyExecutionMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJourneyExecutionMetricsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionMetricsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionMetricsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            GetJourneyExecutionMetricsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionMetricsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJourneyExecutionMetricsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJourneyExecutionMetricsError {}
/// Errors returned by GetPushTemplate
#[derive(Debug, PartialEq)]
pub enum GetPushTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetPushTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPushTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPushTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPushTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetPushTemplateError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetPushTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPushTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetPushTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPushTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPushTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPushTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPushTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetPushTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetPushTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPushTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPushTemplateError {}
/// Errors returned by GetSegment
#[derive(Debug, PartialEq)]
pub enum GetSegmentError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentError {}
/// Errors returned by GetSegmentExportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentExportJobsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentExportJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentExportJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentExportJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentExportJobsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentExportJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentExportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentExportJobsError {}
/// Errors returned by GetSegmentImportJobs
#[derive(Debug, PartialEq)]
pub enum GetSegmentImportJobsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentImportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentImportJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentImportJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentImportJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentImportJobsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentImportJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentImportJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentImportJobsError {}
/// Errors returned by GetSegmentVersion
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentVersionError {}
/// Errors returned by GetSegmentVersions
#[derive(Debug, PartialEq)]
pub enum GetSegmentVersionsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentVersionsError {}
/// Errors returned by GetSegments
#[derive(Debug, PartialEq)]
pub enum GetSegmentsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSegmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSegmentsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSegmentsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSegmentsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSegmentsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSegmentsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSegmentsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSegmentsError {}
/// Errors returned by GetSmsChannel
#[derive(Debug, PartialEq)]
pub enum GetSmsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSmsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSmsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSmsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSmsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSmsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSmsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSmsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSmsChannelError {}
/// Errors returned by GetSmsTemplate
#[derive(Debug, PartialEq)]
pub enum GetSmsTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetSmsTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSmsTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSmsTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetSmsTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetSmsTemplateError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetSmsTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSmsTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSmsTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSmsTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSmsTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetSmsTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetSmsTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetSmsTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetSmsTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetSmsTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSmsTemplateError {}
/// Errors returned by GetUserEndpoints
#[derive(Debug, PartialEq)]
pub enum GetUserEndpointsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetUserEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetUserEndpointsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetUserEndpointsError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetUserEndpointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetUserEndpointsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetUserEndpointsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetUserEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetUserEndpointsError {}
/// Errors returned by GetVoiceChannel
#[derive(Debug, PartialEq)]
pub enum GetVoiceChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetVoiceChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetVoiceChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVoiceChannelError {}
/// Errors returned by GetVoiceTemplate
#[derive(Debug, PartialEq)]
pub enum GetVoiceTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl GetVoiceTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVoiceTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVoiceTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetVoiceTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetVoiceTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(GetVoiceTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVoiceTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetVoiceTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVoiceTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVoiceTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVoiceTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetVoiceTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetVoiceTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            GetVoiceTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetVoiceTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVoiceTemplateError {}
/// Errors returned by ListJourneys
#[derive(Debug, PartialEq)]
pub enum ListJourneysError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl ListJourneysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJourneysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJourneysError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListJourneysError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListJourneysError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(ListJourneysError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJourneysError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListJourneysError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJourneysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJourneysError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJourneysError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListJourneysError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListJourneysError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            ListJourneysError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJourneysError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJourneysError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTemplateVersions
#[derive(Debug, PartialEq)]
pub enum ListTemplateVersionsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl ListTemplateVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplateVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTemplateVersionsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTemplateVersionsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTemplateVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(ListTemplateVersionsError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTemplateVersionsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTemplateVersionsError::TooManyRequests(
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
impl fmt::Display for ListTemplateVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplateVersionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTemplateVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTemplateVersionsError {}
/// Errors returned by ListTemplates
#[derive(Debug, PartialEq)]
pub enum ListTemplatesError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl ListTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTemplatesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTemplatesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTemplatesError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(ListTemplatesError::MethodNotAllowed(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTemplatesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTemplatesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            ListTemplatesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTemplatesError {}
/// Errors returned by PhoneNumberValidate
#[derive(Debug, PartialEq)]
pub enum PhoneNumberValidateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PhoneNumberValidateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PhoneNumberValidateError::BadRequest(ref cause) => write!(f, "{}", cause),
            PhoneNumberValidateError::Forbidden(ref cause) => write!(f, "{}", cause),
            PhoneNumberValidateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PhoneNumberValidateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            PhoneNumberValidateError::NotFound(ref cause) => write!(f, "{}", cause),
            PhoneNumberValidateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PhoneNumberValidateError {}
/// Errors returned by PutEventStream
#[derive(Debug, PartialEq)]
pub enum PutEventStreamError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventStreamError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutEventStreamError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutEventStreamError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutEventStreamError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            PutEventStreamError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEventStreamError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventStreamError {}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutEventsError::Forbidden(ref cause) => write!(f, "{}", cause),
            PutEventsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PutEventsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            PutEventsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEventsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsError {}
/// Errors returned by RemoveAttributes
#[derive(Debug, PartialEq)]
pub enum RemoveAttributesError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            RemoveAttributesError::Forbidden(ref cause) => write!(f, "{}", cause),
            RemoveAttributesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveAttributesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            RemoveAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            RemoveAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveAttributesError {}
/// Errors returned by SendMessages
#[derive(Debug, PartialEq)]
pub enum SendMessagesError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendMessagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendMessagesError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendMessagesError::Forbidden(ref cause) => write!(f, "{}", cause),
            SendMessagesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SendMessagesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            SendMessagesError::NotFound(ref cause) => write!(f, "{}", cause),
            SendMessagesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendMessagesError {}
/// Errors returned by SendUsersMessages
#[derive(Debug, PartialEq)]
pub enum SendUsersMessagesError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendUsersMessagesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendUsersMessagesError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendUsersMessagesError::Forbidden(ref cause) => write!(f, "{}", cause),
            SendUsersMessagesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SendUsersMessagesError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            SendUsersMessagesError::NotFound(ref cause) => write!(f, "{}", cause),
            SendUsersMessagesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendUsersMessagesError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAdmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateAdmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAdmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAdmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateAdmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateAdmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateAdmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateAdmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAdmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAdmChannelError {}
/// Errors returned by UpdateApnsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApnsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApnsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApnsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApnsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateApnsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateApnsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApnsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApnsChannelError {}
/// Errors returned by UpdateApnsSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApnsSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApnsSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApnsSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApnsSandboxChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateApnsSandboxChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateApnsSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApnsSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApnsSandboxChannelError {}
/// Errors returned by UpdateApnsVoipChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApnsVoipChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApnsVoipChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApnsVoipChannelError {}
/// Errors returned by UpdateApnsVoipSandboxChannel
#[derive(Debug, PartialEq)]
pub enum UpdateApnsVoipSandboxChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApnsVoipSandboxChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApnsVoipSandboxChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipSandboxChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipSandboxChannelError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApnsVoipSandboxChannelError::MethodNotAllowed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApnsVoipSandboxChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApnsVoipSandboxChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApnsVoipSandboxChannelError {}
/// Errors returned by UpdateApplicationSettings
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationSettingsError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApplicationSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationSettingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateApplicationSettingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateApplicationSettingsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationSettingsError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateApplicationSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateApplicationSettingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationSettingsError {}
/// Errors returned by UpdateBaiduChannel
#[derive(Debug, PartialEq)]
pub enum UpdateBaiduChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBaiduChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBaiduChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateBaiduChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateBaiduChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateBaiduChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateBaiduChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateBaiduChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBaiduChannelError {}
/// Errors returned by UpdateCampaign
#[derive(Debug, PartialEq)]
pub enum UpdateCampaignError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCampaignError {}
/// Errors returned by UpdateEmailChannel
#[derive(Debug, PartialEq)]
pub enum UpdateEmailChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEmailChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEmailChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEmailChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEmailChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateEmailChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateEmailChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEmailChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEmailChannelError {}
/// Errors returned by UpdateEmailTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateEmailTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateEmailTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEmailTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEmailTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEmailTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEmailTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEmailTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEmailTemplateError {}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEndpointError {}
/// Errors returned by UpdateEndpointsBatch
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointsBatchError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateEndpointsBatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointsBatchError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateEndpointsBatchError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateEndpointsBatchError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateEndpointsBatchError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateEndpointsBatchError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateEndpointsBatchError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEndpointsBatchError {}
/// Errors returned by UpdateGcmChannel
#[derive(Debug, PartialEq)]
pub enum UpdateGcmChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGcmChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGcmChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateGcmChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateGcmChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGcmChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateGcmChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGcmChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGcmChannelError {}
/// Errors returned by UpdateJourney
#[derive(Debug, PartialEq)]
pub enum UpdateJourneyError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateJourneyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJourneyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateJourneyError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateJourneyError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateJourneyError::InternalServerError(err.msg))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateJourneyError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateJourneyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateJourneyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJourneyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJourneyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateJourneyError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateJourneyError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateJourneyError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateJourneyError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateJourneyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJourneyError {}
/// Errors returned by UpdateJourneyState
#[derive(Debug, PartialEq)]
pub enum UpdateJourneyStateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateJourneyStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJourneyStateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateJourneyStateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateJourneyStateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateJourneyStateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateJourneyStateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateJourneyStateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateJourneyStateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJourneyStateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJourneyStateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateJourneyStateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateJourneyStateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateJourneyStateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateJourneyStateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateJourneyStateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJourneyStateError {}
/// Errors returned by UpdatePushTemplate
#[derive(Debug, PartialEq)]
pub enum UpdatePushTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdatePushTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePushTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePushTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePushTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdatePushTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdatePushTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePushTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePushTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePushTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePushTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePushTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePushTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdatePushTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdatePushTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePushTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePushTemplateError {}
/// Errors returned by UpdateSegment
#[derive(Debug, PartialEq)]
pub enum UpdateSegmentError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSegmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSegmentError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateSegmentError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateSegmentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSegmentError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateSegmentError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateSegmentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSegmentError {}
/// Errors returned by UpdateSmsChannel
#[derive(Debug, PartialEq)]
pub enum UpdateSmsChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSmsChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSmsChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateSmsChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateSmsChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSmsChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateSmsChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateSmsChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSmsChannelError {}
/// Errors returned by UpdateSmsTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateSmsTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateSmsTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSmsTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::MethodNotAllowed(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateSmsTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSmsTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSmsTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateSmsTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateSmsTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSmsTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateSmsTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateSmsTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSmsTemplateError {}
/// Errors returned by UpdateTemplateActiveVersion
#[derive(Debug, PartialEq)]
pub enum UpdateTemplateActiveVersionError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateTemplateActiveVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateTemplateActiveVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateTemplateActiveVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateTemplateActiveVersionError::Forbidden(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateTemplateActiveVersionError::InternalServerError(err.msg),
                    )
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(
                        UpdateTemplateActiveVersionError::MethodNotAllowed(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateTemplateActiveVersionError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateTemplateActiveVersionError::TooManyRequests(
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
impl fmt::Display for UpdateTemplateActiveVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTemplateActiveVersionError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateTemplateActiveVersionError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateTemplateActiveVersionError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTemplateActiveVersionError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateTemplateActiveVersionError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateTemplateActiveVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTemplateActiveVersionError {}
/// Errors returned by UpdateVoiceChannel
#[derive(Debug, PartialEq)]
pub enum UpdateVoiceChannelError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVoiceChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVoiceChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVoiceChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVoiceChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVoiceChannelError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateVoiceChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVoiceChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVoiceChannelError {}
/// Errors returned by UpdateVoiceTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateVoiceTemplateError {
    /// <p>Provides information about an API request or response.</p>
    BadRequest(String),
    /// <p>Provides information about an API request or response.</p>
    Forbidden(String),
    /// <p>Provides information about an API request or response.</p>
    InternalServerError(String),
    /// <p>Provides information about an API request or response.</p>
    MethodNotAllowed(String),
    /// <p>Provides information about an API request or response.</p>
    NotFound(String),
    /// <p>Provides information about an API request or response.</p>
    TooManyRequests(String),
}

impl UpdateVoiceTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVoiceTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "MethodNotAllowedException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::MethodNotAllowed(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVoiceTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVoiceTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVoiceTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVoiceTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateVoiceTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVoiceTemplateError::MethodNotAllowed(ref cause) => write!(f, "{}", cause),
            UpdateVoiceTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateVoiceTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVoiceTemplateError {}
/// Trait representing the capabilities of the Amazon Pinpoint API. Amazon Pinpoint clients implement this trait.
#[async_trait]
pub trait Pinpoint {
    /// <p><p>Creates an application.</p></p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>>;

    /// <p>Creates a new campaign for an application or updates the settings of an existing campaign for an application.</p>
    async fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> Result<CreateCampaignResponse, RusotoError<CreateCampaignError>>;

    /// <p>Creates a message template for messages that are sent through the email channel.</p>
    async fn create_email_template(
        &self,
        input: CreateEmailTemplateRequest,
    ) -> Result<CreateEmailTemplateResponse, RusotoError<CreateEmailTemplateError>>;

    /// <p>Creates an export job for an application.</p>
    async fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> Result<CreateExportJobResponse, RusotoError<CreateExportJobError>>;

    /// <p>Creates an import job for an application.</p>
    async fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> Result<CreateImportJobResponse, RusotoError<CreateImportJobError>>;

    /// <p>Creates a journey for an application.</p>
    async fn create_journey(
        &self,
        input: CreateJourneyRequest,
    ) -> Result<CreateJourneyResponse, RusotoError<CreateJourneyError>>;

    /// <p>Creates a message template for messages that are sent through a push notification channel.</p>
    async fn create_push_template(
        &self,
        input: CreatePushTemplateRequest,
    ) -> Result<CreatePushTemplateResponse, RusotoError<CreatePushTemplateError>>;

    /// <p>Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application.</p>
    async fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> Result<CreateSegmentResponse, RusotoError<CreateSegmentError>>;

    /// <p>Creates a message template for messages that are sent through the SMS channel.</p>
    async fn create_sms_template(
        &self,
        input: CreateSmsTemplateRequest,
    ) -> Result<CreateSmsTemplateResponse, RusotoError<CreateSmsTemplateError>>;

    /// <p>Creates a message template for messages that are sent through the voice channel.</p>
    async fn create_voice_template(
        &self,
        input: CreateVoiceTemplateRequest,
    ) -> Result<CreateVoiceTemplateResponse, RusotoError<CreateVoiceTemplateError>>;

    /// <p>Disables the ADM channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> Result<DeleteAdmChannelResponse, RusotoError<DeleteAdmChannelError>>;

    /// <p>Disables the APNs channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> Result<DeleteApnsChannelResponse, RusotoError<DeleteApnsChannelError>>;

    /// <p>Disables the APNs sandbox channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> Result<DeleteApnsSandboxChannelResponse, RusotoError<DeleteApnsSandboxChannelError>>;

    /// <p>Disables the APNs VoIP channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> Result<DeleteApnsVoipChannelResponse, RusotoError<DeleteApnsVoipChannelError>>;

    /// <p>Disables the APNs VoIP sandbox channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> Result<DeleteApnsVoipSandboxChannelResponse, RusotoError<DeleteApnsVoipSandboxChannelError>>;

    /// <p>Deletes an application.</p>
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResponse, RusotoError<DeleteAppError>>;

    /// <p>Disables the Baidu channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> Result<DeleteBaiduChannelResponse, RusotoError<DeleteBaiduChannelError>>;

    /// <p>Deletes a campaign from an application.</p>
    async fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> Result<DeleteCampaignResponse, RusotoError<DeleteCampaignError>>;

    /// <p>Disables the email channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> Result<DeleteEmailChannelResponse, RusotoError<DeleteEmailChannelError>>;

    /// <p>Deletes a message template for messages that were sent through the email channel.</p>
    async fn delete_email_template(
        &self,
        input: DeleteEmailTemplateRequest,
    ) -> Result<DeleteEmailTemplateResponse, RusotoError<DeleteEmailTemplateError>>;

    /// <p>Deletes an endpoint from an application.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>>;

    /// <p>Deletes the event stream for an application.</p>
    async fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> Result<DeleteEventStreamResponse, RusotoError<DeleteEventStreamError>>;

    /// <p>Disables the GCM channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> Result<DeleteGcmChannelResponse, RusotoError<DeleteGcmChannelError>>;

    /// <p>Deletes a journey from an application.</p>
    async fn delete_journey(
        &self,
        input: DeleteJourneyRequest,
    ) -> Result<DeleteJourneyResponse, RusotoError<DeleteJourneyError>>;

    /// <p>Deletes a message template for messages that were sent through a push notification channel.</p>
    async fn delete_push_template(
        &self,
        input: DeletePushTemplateRequest,
    ) -> Result<DeletePushTemplateResponse, RusotoError<DeletePushTemplateError>>;

    /// <p>Deletes a segment from an application.</p>
    async fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> Result<DeleteSegmentResponse, RusotoError<DeleteSegmentError>>;

    /// <p>Disables the SMS channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> Result<DeleteSmsChannelResponse, RusotoError<DeleteSmsChannelError>>;

    /// <p>Deletes a message template for messages that were sent through the SMS channel.</p>
    async fn delete_sms_template(
        &self,
        input: DeleteSmsTemplateRequest,
    ) -> Result<DeleteSmsTemplateResponse, RusotoError<DeleteSmsTemplateError>>;

    /// <p>Deletes all the endpoints that are associated with a specific user ID.</p>
    async fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> Result<DeleteUserEndpointsResponse, RusotoError<DeleteUserEndpointsError>>;

    /// <p>Disables the voice channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_voice_channel(
        &self,
        input: DeleteVoiceChannelRequest,
    ) -> Result<DeleteVoiceChannelResponse, RusotoError<DeleteVoiceChannelError>>;

    /// <p>Deletes a message template for messages that were sent through the voice channel.</p>
    async fn delete_voice_template(
        &self,
        input: DeleteVoiceTemplateRequest,
    ) -> Result<DeleteVoiceTemplateResponse, RusotoError<DeleteVoiceTemplateError>>;

    /// <p>Retrieves information about the status and settings of the ADM channel for an application.</p>
    async fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> Result<GetAdmChannelResponse, RusotoError<GetAdmChannelError>>;

    /// <p>Retrieves information about the status and settings of the APNs channel for an application.</p>
    async fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> Result<GetApnsChannelResponse, RusotoError<GetApnsChannelError>>;

    /// <p>Retrieves information about the status and settings of the APNs sandbox channel for an application.</p>
    async fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> Result<GetApnsSandboxChannelResponse, RusotoError<GetApnsSandboxChannelError>>;

    /// <p>Retrieves information about the status and settings of the APNs VoIP channel for an application.</p>
    async fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> Result<GetApnsVoipChannelResponse, RusotoError<GetApnsVoipChannelError>>;

    /// <p>Retrieves information about the status and settings of the APNs VoIP sandbox channel for an application.</p>
    async fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> Result<GetApnsVoipSandboxChannelResponse, RusotoError<GetApnsVoipSandboxChannelError>>;

    /// <p>Retrieves information about an application.</p>
    async fn get_app(
        &self,
        input: GetAppRequest,
    ) -> Result<GetAppResponse, RusotoError<GetAppError>>;

    /// <p>Retrieves (queries) pre-aggregated data for a standard metric that applies to an application.</p>
    async fn get_application_date_range_kpi(
        &self,
        input: GetApplicationDateRangeKpiRequest,
    ) -> Result<GetApplicationDateRangeKpiResponse, RusotoError<GetApplicationDateRangeKpiError>>;

    /// <p>Retrieves information about the settings for an application.</p>
    async fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> Result<GetApplicationSettingsResponse, RusotoError<GetApplicationSettingsError>>;

    /// <p>Retrieves information about all the applications that are associated with your Amazon Pinpoint account.</p>
    async fn get_apps(
        &self,
        input: GetAppsRequest,
    ) -> Result<GetAppsResponse, RusotoError<GetAppsError>>;

    /// <p>Retrieves information about the status and settings of the Baidu channel for an application.</p>
    async fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> Result<GetBaiduChannelResponse, RusotoError<GetBaiduChannelError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for a campaign.</p>
    async fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> Result<GetCampaignResponse, RusotoError<GetCampaignError>>;

    /// <p>Retrieves information about all the activities for a campaign.</p>
    async fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> Result<GetCampaignActivitiesResponse, RusotoError<GetCampaignActivitiesError>>;

    /// <p>Retrieves (queries) pre-aggregated data for a standard metric that applies to a campaign.</p>
    async fn get_campaign_date_range_kpi(
        &self,
        input: GetCampaignDateRangeKpiRequest,
    ) -> Result<GetCampaignDateRangeKpiResponse, RusotoError<GetCampaignDateRangeKpiError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for a specific version of a campaign.</p>
    async fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> Result<GetCampaignVersionResponse, RusotoError<GetCampaignVersionError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for all versions of a campaign.</p>
    async fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> Result<GetCampaignVersionsResponse, RusotoError<GetCampaignVersionsError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for all the campaigns that are associated with an application.</p>
    async fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> Result<GetCampaignsResponse, RusotoError<GetCampaignsError>>;

    /// <p>Retrieves information about the history and status of each channel for an application.</p>
    async fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> Result<GetChannelsResponse, RusotoError<GetChannelsError>>;

    /// <p>Retrieves information about the status and settings of the email channel for an application.</p>
    async fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> Result<GetEmailChannelResponse, RusotoError<GetEmailChannelError>>;

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the email channel.</p>
    async fn get_email_template(
        &self,
        input: GetEmailTemplateRequest,
    ) -> Result<GetEmailTemplateResponse, RusotoError<GetEmailTemplateError>>;

    /// <p>Retrieves information about the settings and attributes of a specific endpoint for an application.</p>
    async fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> Result<GetEndpointResponse, RusotoError<GetEndpointError>>;

    /// <p>Retrieves information about the event stream settings for an application.</p>
    async fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> Result<GetEventStreamResponse, RusotoError<GetEventStreamError>>;

    /// <p>Retrieves information about the status and settings of a specific export job for an application.</p>
    async fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> Result<GetExportJobResponse, RusotoError<GetExportJobError>>;

    /// <p>Retrieves information about the status and settings of all the export jobs for an application.</p>
    async fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> Result<GetExportJobsResponse, RusotoError<GetExportJobsError>>;

    /// <p>Retrieves information about the status and settings of the GCM channel for an application.</p>
    async fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> Result<GetGcmChannelResponse, RusotoError<GetGcmChannelError>>;

    /// <p>Retrieves information about the status and settings of a specific import job for an application.</p>
    async fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> Result<GetImportJobResponse, RusotoError<GetImportJobError>>;

    /// <p>Retrieves information about the status and settings of all the import jobs for an application.</p>
    async fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> Result<GetImportJobsResponse, RusotoError<GetImportJobsError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for a journey.</p>
    async fn get_journey(
        &self,
        input: GetJourneyRequest,
    ) -> Result<GetJourneyResponse, RusotoError<GetJourneyError>>;

    /// <p>Retrieves (queries) pre-aggregated data for a standard engagement metric that applies to a journey.</p>
    async fn get_journey_date_range_kpi(
        &self,
        input: GetJourneyDateRangeKpiRequest,
    ) -> Result<GetJourneyDateRangeKpiResponse, RusotoError<GetJourneyDateRangeKpiError>>;

    /// <p>Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey activity.</p>
    async fn get_journey_execution_activity_metrics(
        &self,
        input: GetJourneyExecutionActivityMetricsRequest,
    ) -> Result<
        GetJourneyExecutionActivityMetricsResponse,
        RusotoError<GetJourneyExecutionActivityMetricsError>,
    >;

    /// <p>Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey.</p>
    async fn get_journey_execution_metrics(
        &self,
        input: GetJourneyExecutionMetricsRequest,
    ) -> Result<GetJourneyExecutionMetricsResponse, RusotoError<GetJourneyExecutionMetricsError>>;

    /// <p>Retrieves the content and settings of a message template for messages that are sent through a push notification channel.</p>
    async fn get_push_template(
        &self,
        input: GetPushTemplateRequest,
    ) -> Result<GetPushTemplateResponse, RusotoError<GetPushTemplateError>>;

    /// <p>Retrieves information about the configuration, dimension, and other settings for a specific segment that's associated with an application.</p>
    async fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> Result<GetSegmentResponse, RusotoError<GetSegmentError>>;

    /// <p>Retrieves information about the status and settings of the export jobs for a segment.</p>
    async fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> Result<GetSegmentExportJobsResponse, RusotoError<GetSegmentExportJobsError>>;

    /// <p>Retrieves information about the status and settings of the import jobs for a segment.</p>
    async fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> Result<GetSegmentImportJobsResponse, RusotoError<GetSegmentImportJobsError>>;

    /// <p>Retrieves information about the configuration, dimension, and other settings for a specific version of a segment that's associated with an application.</p>
    async fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> Result<GetSegmentVersionResponse, RusotoError<GetSegmentVersionError>>;

    /// <p>Retrieves information about the configuration, dimension, and other settings for all the versions of a specific segment that's associated with an application.</p>
    async fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> Result<GetSegmentVersionsResponse, RusotoError<GetSegmentVersionsError>>;

    /// <p>Retrieves information about the configuration, dimension, and other settings for all the segments that are associated with an application.</p>
    async fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> Result<GetSegmentsResponse, RusotoError<GetSegmentsError>>;

    /// <p>Retrieves information about the status and settings of the SMS channel for an application.</p>
    async fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> Result<GetSmsChannelResponse, RusotoError<GetSmsChannelError>>;

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the SMS channel.</p>
    async fn get_sms_template(
        &self,
        input: GetSmsTemplateRequest,
    ) -> Result<GetSmsTemplateResponse, RusotoError<GetSmsTemplateError>>;

    /// <p>Retrieves information about all the endpoints that are associated with a specific user ID.</p>
    async fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> Result<GetUserEndpointsResponse, RusotoError<GetUserEndpointsError>>;

    /// <p>Retrieves information about the status and settings of the voice channel for an application.</p>
    async fn get_voice_channel(
        &self,
        input: GetVoiceChannelRequest,
    ) -> Result<GetVoiceChannelResponse, RusotoError<GetVoiceChannelError>>;

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the voice channel.</p>
    async fn get_voice_template(
        &self,
        input: GetVoiceTemplateRequest,
    ) -> Result<GetVoiceTemplateResponse, RusotoError<GetVoiceTemplateError>>;

    /// <p>Retrieves information about the status, configuration, and other settings for all the journeys that are associated with an application.</p>
    async fn list_journeys(
        &self,
        input: ListJourneysRequest,
    ) -> Result<ListJourneysResponse, RusotoError<ListJourneysError>>;

    /// <p>Retrieves all the tags (keys and values) that are associated with an application, campaign, journey, message template, or segment.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Retrieves information about all the versions of a specific message template.</p>
    async fn list_template_versions(
        &self,
        input: ListTemplateVersionsRequest,
    ) -> Result<ListTemplateVersionsResponse, RusotoError<ListTemplateVersionsError>>;

    /// <p>Retrieves information about all the message templates that are associated with your Amazon Pinpoint account.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>>;

    /// <p>Retrieves information about a phone number.</p>
    async fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> Result<PhoneNumberValidateResponse, RusotoError<PhoneNumberValidateError>>;

    /// <p>Creates a new event stream for an application or updates the settings of an existing event stream for an application.</p>
    async fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> Result<PutEventStreamResponse, RusotoError<PutEventStreamError>>;

    /// <p>Creates a new event to record for endpoints, or creates or updates endpoint data that existing events are associated with.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>>;

    /// <p>Removes one or more attributes, of the same attribute type, from all the endpoints that are associated with an application.</p>
    async fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> Result<RemoveAttributesResponse, RusotoError<RemoveAttributesError>>;

    /// <p>Creates and sends a direct message.</p>
    async fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> Result<SendMessagesResponse, RusotoError<SendMessagesError>>;

    /// <p>Creates and sends a message to a list of users.</p>
    async fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> Result<SendUsersMessagesResponse, RusotoError<SendUsersMessagesError>>;

    /// <p>Adds one or more tags (keys and values) to an application, campaign, journey, message template, or segment.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags (keys and values) from an application, campaign, journey, message template, or segment.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Enables the ADM channel for an application or updates the status and settings of the ADM channel for an application.</p>
    async fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> Result<UpdateAdmChannelResponse, RusotoError<UpdateAdmChannelError>>;

    /// <p>Enables the APNs channel for an application or updates the status and settings of the APNs channel for an application.</p>
    async fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> Result<UpdateApnsChannelResponse, RusotoError<UpdateApnsChannelError>>;

    /// <p>Enables the APNs sandbox channel for an application or updates the status and settings of the APNs sandbox channel for an application.</p>
    async fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> Result<UpdateApnsSandboxChannelResponse, RusotoError<UpdateApnsSandboxChannelError>>;

    /// <p>Enables the APNs VoIP channel for an application or updates the status and settings of the APNs VoIP channel for an application.</p>
    async fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> Result<UpdateApnsVoipChannelResponse, RusotoError<UpdateApnsVoipChannelError>>;

    /// <p>Enables the APNs VoIP sandbox channel for an application or updates the status and settings of the APNs VoIP sandbox channel for an application.</p>
    async fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> Result<UpdateApnsVoipSandboxChannelResponse, RusotoError<UpdateApnsVoipSandboxChannelError>>;

    /// <p>Updates the settings for an application.</p>
    async fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> Result<UpdateApplicationSettingsResponse, RusotoError<UpdateApplicationSettingsError>>;

    /// <p>Enables the Baidu channel for an application or updates the status and settings of the Baidu channel for an application.</p>
    async fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> Result<UpdateBaiduChannelResponse, RusotoError<UpdateBaiduChannelError>>;

    /// <p>Updates the configuration and other settings for a campaign.</p>
    async fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> Result<UpdateCampaignResponse, RusotoError<UpdateCampaignError>>;

    /// <p>Enables the email channel for an application or updates the status and settings of the email channel for an application.</p>
    async fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> Result<UpdateEmailChannelResponse, RusotoError<UpdateEmailChannelError>>;

    /// <p>Updates an existing message template for messages that are sent through the email channel.</p>
    async fn update_email_template(
        &self,
        input: UpdateEmailTemplateRequest,
    ) -> Result<UpdateEmailTemplateResponse, RusotoError<UpdateEmailTemplateError>>;

    /// <p>Creates a new endpoint for an application or updates the settings and attributes of an existing endpoint for an application. You can also use this operation to define custom attributes (Attributes, Metrics, and UserAttributes properties) for an endpoint.</p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> Result<UpdateEndpointResponse, RusotoError<UpdateEndpointError>>;

    /// <p><p>Creates a new batch of endpoints for an application or updates the settings and attributes of a batch of existing endpoints for an application. You can also use this operation to define custom attributes (Attributes, Metrics, and UserAttributes properties) for a batch of endpoints.</p></p>
    async fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> Result<UpdateEndpointsBatchResponse, RusotoError<UpdateEndpointsBatchError>>;

    /// <p>Enables the GCM channel for an application or updates the status and settings of the GCM channel for an application.</p>
    async fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> Result<UpdateGcmChannelResponse, RusotoError<UpdateGcmChannelError>>;

    /// <p>Updates the configuration and other settings for a journey.</p>
    async fn update_journey(
        &self,
        input: UpdateJourneyRequest,
    ) -> Result<UpdateJourneyResponse, RusotoError<UpdateJourneyError>>;

    /// <p>Cancels (stops) an active journey.</p>
    async fn update_journey_state(
        &self,
        input: UpdateJourneyStateRequest,
    ) -> Result<UpdateJourneyStateResponse, RusotoError<UpdateJourneyStateError>>;

    /// <p>Updates an existing message template for messages that are sent through a push notification channel.</p>
    async fn update_push_template(
        &self,
        input: UpdatePushTemplateRequest,
    ) -> Result<UpdatePushTemplateResponse, RusotoError<UpdatePushTemplateError>>;

    /// <p>Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application.</p>
    async fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> Result<UpdateSegmentResponse, RusotoError<UpdateSegmentError>>;

    /// <p>Enables the SMS channel for an application or updates the status and settings of the SMS channel for an application.</p>
    async fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> Result<UpdateSmsChannelResponse, RusotoError<UpdateSmsChannelError>>;

    /// <p>Updates an existing message template for messages that are sent through the SMS channel.</p>
    async fn update_sms_template(
        &self,
        input: UpdateSmsTemplateRequest,
    ) -> Result<UpdateSmsTemplateResponse, RusotoError<UpdateSmsTemplateError>>;

    /// <p>Changes the status of a specific version of a message template to <i>active</i>.</p>
    async fn update_template_active_version(
        &self,
        input: UpdateTemplateActiveVersionRequest,
    ) -> Result<UpdateTemplateActiveVersionResponse, RusotoError<UpdateTemplateActiveVersionError>>;

    /// <p>Enables the voice channel for an application or updates the status and settings of the voice channel for an application.</p>
    async fn update_voice_channel(
        &self,
        input: UpdateVoiceChannelRequest,
    ) -> Result<UpdateVoiceChannelResponse, RusotoError<UpdateVoiceChannelError>>;

    /// <p>Updates an existing message template for messages that are sent through the voice channel.</p>
    async fn update_voice_template(
        &self,
        input: UpdateVoiceTemplateRequest,
    ) -> Result<UpdateVoiceTemplateResponse, RusotoError<UpdateVoiceTemplateError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PinpointClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PinpointClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PinpointClient {
        PinpointClient { client, region }
    }
}

#[async_trait]
impl Pinpoint for PinpointClient {
    /// <p><p>Creates an application.</p></p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>> {
        let request_uri = "/v1/apps";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.create_application_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateAppResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAppError::from_response(response))
        }
    }

    /// <p>Creates a new campaign for an application or updates the settings of an existing campaign for an application.</p>
    async fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> Result<CreateCampaignResponse, RusotoError<CreateCampaignError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_campaign_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCampaignError::from_response(response))
        }
    }

    /// <p>Creates a message template for messages that are sent through the email channel.</p>
    async fn create_email_template(
        &self,
        input: CreateEmailTemplateRequest,
    ) -> Result<CreateEmailTemplateResponse, RusotoError<CreateEmailTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/email",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_template_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEmailTemplateError::from_response(response))
        }
    }

    /// <p>Creates an export job for an application.</p>
    async fn create_export_job(
        &self,
        input: CreateExportJobRequest,
    ) -> Result<CreateExportJobResponse, RusotoError<CreateExportJobError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.export_job_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateExportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateExportJobError::from_response(response))
        }
    }

    /// <p>Creates an import job for an application.</p>
    async fn create_import_job(
        &self,
        input: CreateImportJobRequest,
    ) -> Result<CreateImportJobResponse, RusotoError<CreateImportJobError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.import_job_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateImportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateImportJobError::from_response(response))
        }
    }

    /// <p>Creates a journey for an application.</p>
    async fn create_journey(
        &self,
        input: CreateJourneyRequest,
    ) -> Result<CreateJourneyResponse, RusotoError<CreateJourneyError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_journey_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateJourneyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJourneyError::from_response(response))
        }
    }

    /// <p>Creates a message template for messages that are sent through a push notification channel.</p>
    async fn create_push_template(
        &self,
        input: CreatePushTemplateRequest,
    ) -> Result<CreatePushTemplateResponse, RusotoError<CreatePushTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/push",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.push_notification_template_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePushTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePushTemplateError::from_response(response))
        }
    }

    /// <p>Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application.</p>
    async fn create_segment(
        &self,
        input: CreateSegmentRequest,
    ) -> Result<CreateSegmentResponse, RusotoError<CreateSegmentError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_segment_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSegmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSegmentError::from_response(response))
        }
    }

    /// <p>Creates a message template for messages that are sent through the SMS channel.</p>
    async fn create_sms_template(
        &self,
        input: CreateSmsTemplateRequest,
    ) -> Result<CreateSmsTemplateResponse, RusotoError<CreateSmsTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/sms",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_template_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateSmsTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateSmsTemplateError::from_response(response))
        }
    }

    /// <p>Creates a message template for messages that are sent through the voice channel.</p>
    async fn create_voice_template(
        &self,
        input: CreateVoiceTemplateRequest,
    ) -> Result<CreateVoiceTemplateResponse, RusotoError<CreateVoiceTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/voice",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.voice_template_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVoiceTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVoiceTemplateError::from_response(response))
        }
    }

    /// <p>Disables the ADM channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_adm_channel(
        &self,
        input: DeleteAdmChannelRequest,
    ) -> Result<DeleteAdmChannelResponse, RusotoError<DeleteAdmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAdmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAdmChannelError::from_response(response))
        }
    }

    /// <p>Disables the APNs channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_channel(
        &self,
        input: DeleteApnsChannelRequest,
    ) -> Result<DeleteApnsChannelResponse, RusotoError<DeleteApnsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteApnsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApnsChannelError::from_response(response))
        }
    }

    /// <p>Disables the APNs sandbox channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_sandbox_channel(
        &self,
        input: DeleteApnsSandboxChannelRequest,
    ) -> Result<DeleteApnsSandboxChannelResponse, RusotoError<DeleteApnsSandboxChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteApnsSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApnsSandboxChannelError::from_response(response))
        }
    }

    /// <p>Disables the APNs VoIP channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_voip_channel(
        &self,
        input: DeleteApnsVoipChannelRequest,
    ) -> Result<DeleteApnsVoipChannelResponse, RusotoError<DeleteApnsVoipChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteApnsVoipChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApnsVoipChannelError::from_response(response))
        }
    }

    /// <p>Disables the APNs VoIP sandbox channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_apns_voip_sandbox_channel(
        &self,
        input: DeleteApnsVoipSandboxChannelRequest,
    ) -> Result<DeleteApnsVoipSandboxChannelResponse, RusotoError<DeleteApnsVoipSandboxChannelError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteApnsVoipSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApnsVoipSandboxChannelError::from_response(response))
        }
    }

    /// <p>Deletes an application.</p>
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResponse, RusotoError<DeleteAppError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteAppResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAppError::from_response(response))
        }
    }

    /// <p>Disables the Baidu channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_baidu_channel(
        &self,
        input: DeleteBaiduChannelRequest,
    ) -> Result<DeleteBaiduChannelResponse, RusotoError<DeleteBaiduChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteBaiduChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteBaiduChannelError::from_response(response))
        }
    }

    /// <p>Deletes a campaign from an application.</p>
    async fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> Result<DeleteCampaignResponse, RusotoError<DeleteCampaignError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCampaignError::from_response(response))
        }
    }

    /// <p>Disables the email channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_email_channel(
        &self,
        input: DeleteEmailChannelRequest,
    ) -> Result<DeleteEmailChannelResponse, RusotoError<DeleteEmailChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailChannelError::from_response(response))
        }
    }

    /// <p>Deletes a message template for messages that were sent through the email channel.</p>
    async fn delete_email_template(
        &self,
        input: DeleteEmailTemplateRequest,
    ) -> Result<DeleteEmailTemplateResponse, RusotoError<DeleteEmailTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/email",
            template_name = input.template_name
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailTemplateError::from_response(response))
        }
    }

    /// <p>Deletes an endpoint from an application.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEndpointError::from_response(response))
        }
    }

    /// <p>Deletes the event stream for an application.</p>
    async fn delete_event_stream(
        &self,
        input: DeleteEventStreamRequest,
    ) -> Result<DeleteEventStreamResponse, RusotoError<DeleteEventStreamError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEventStreamResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEventStreamError::from_response(response))
        }
    }

    /// <p>Disables the GCM channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_gcm_channel(
        &self,
        input: DeleteGcmChannelRequest,
    ) -> Result<DeleteGcmChannelResponse, RusotoError<DeleteGcmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteGcmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteGcmChannelError::from_response(response))
        }
    }

    /// <p>Deletes a journey from an application.</p>
    async fn delete_journey(
        &self,
        input: DeleteJourneyRequest,
    ) -> Result<DeleteJourneyResponse, RusotoError<DeleteJourneyError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}",
            application_id = input.application_id,
            journey_id = input.journey_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteJourneyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJourneyError::from_response(response))
        }
    }

    /// <p>Deletes a message template for messages that were sent through a push notification channel.</p>
    async fn delete_push_template(
        &self,
        input: DeletePushTemplateRequest,
    ) -> Result<DeletePushTemplateResponse, RusotoError<DeletePushTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/push",
            template_name = input.template_name
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePushTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePushTemplateError::from_response(response))
        }
    }

    /// <p>Deletes a segment from an application.</p>
    async fn delete_segment(
        &self,
        input: DeleteSegmentRequest,
    ) -> Result<DeleteSegmentResponse, RusotoError<DeleteSegmentError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSegmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSegmentError::from_response(response))
        }
    }

    /// <p>Disables the SMS channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_sms_channel(
        &self,
        input: DeleteSmsChannelRequest,
    ) -> Result<DeleteSmsChannelResponse, RusotoError<DeleteSmsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSmsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSmsChannelError::from_response(response))
        }
    }

    /// <p>Deletes a message template for messages that were sent through the SMS channel.</p>
    async fn delete_sms_template(
        &self,
        input: DeleteSmsTemplateRequest,
    ) -> Result<DeleteSmsTemplateResponse, RusotoError<DeleteSmsTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/sms",
            template_name = input.template_name
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSmsTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSmsTemplateError::from_response(response))
        }
    }

    /// <p>Deletes all the endpoints that are associated with a specific user ID.</p>
    async fn delete_user_endpoints(
        &self,
        input: DeleteUserEndpointsRequest,
    ) -> Result<DeleteUserEndpointsResponse, RusotoError<DeleteUserEndpointsError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteUserEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteUserEndpointsError::from_response(response))
        }
    }

    /// <p>Disables the voice channel for an application and deletes any existing settings for the channel.</p>
    async fn delete_voice_channel(
        &self,
        input: DeleteVoiceChannelRequest,
    ) -> Result<DeleteVoiceChannelResponse, RusotoError<DeleteVoiceChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVoiceChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceChannelError::from_response(response))
        }
    }

    /// <p>Deletes a message template for messages that were sent through the voice channel.</p>
    async fn delete_voice_template(
        &self,
        input: DeleteVoiceTemplateRequest,
    ) -> Result<DeleteVoiceTemplateResponse, RusotoError<DeleteVoiceTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/voice",
            template_name = input.template_name
        );

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVoiceTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVoiceTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the ADM channel for an application.</p>
    async fn get_adm_channel(
        &self,
        input: GetAdmChannelRequest,
    ) -> Result<GetAdmChannelResponse, RusotoError<GetAdmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAdmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAdmChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the APNs channel for an application.</p>
    async fn get_apns_channel(
        &self,
        input: GetApnsChannelRequest,
    ) -> Result<GetApnsChannelResponse, RusotoError<GetApnsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApnsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApnsChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the APNs sandbox channel for an application.</p>
    async fn get_apns_sandbox_channel(
        &self,
        input: GetApnsSandboxChannelRequest,
    ) -> Result<GetApnsSandboxChannelResponse, RusotoError<GetApnsSandboxChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApnsSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApnsSandboxChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the APNs VoIP channel for an application.</p>
    async fn get_apns_voip_channel(
        &self,
        input: GetApnsVoipChannelRequest,
    ) -> Result<GetApnsVoipChannelResponse, RusotoError<GetApnsVoipChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApnsVoipChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApnsVoipChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the APNs VoIP sandbox channel for an application.</p>
    async fn get_apns_voip_sandbox_channel(
        &self,
        input: GetApnsVoipSandboxChannelRequest,
    ) -> Result<GetApnsVoipSandboxChannelResponse, RusotoError<GetApnsVoipSandboxChannelError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApnsVoipSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApnsVoipSandboxChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about an application.</p>
    async fn get_app(
        &self,
        input: GetAppRequest,
    ) -> Result<GetAppResponse, RusotoError<GetAppError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetAppResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAppError::from_response(response))
        }
    }

    /// <p>Retrieves (queries) pre-aggregated data for a standard metric that applies to an application.</p>
    async fn get_application_date_range_kpi(
        &self,
        input: GetApplicationDateRangeKpiRequest,
    ) -> Result<GetApplicationDateRangeKpiResponse, RusotoError<GetApplicationDateRangeKpiError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/kpis/daterange/{kpi_name}",
            application_id = input.application_id,
            kpi_name = input.kpi_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("end-time", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("start-time", x);
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
                .deserialize::<GetApplicationDateRangeKpiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationDateRangeKpiError::from_response(response))
        }
    }

    /// <p>Retrieves information about the settings for an application.</p>
    async fn get_application_settings(
        &self,
        input: GetApplicationSettingsRequest,
    ) -> Result<GetApplicationSettingsResponse, RusotoError<GetApplicationSettingsError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetApplicationSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetApplicationSettingsError::from_response(response))
        }
    }

    /// <p>Retrieves information about all the applications that are associated with your Amazon Pinpoint account.</p>
    async fn get_apps(
        &self,
        input: GetAppsRequest,
    ) -> Result<GetAppsResponse, RusotoError<GetAppsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetAppsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAppsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the Baidu channel for an application.</p>
    async fn get_baidu_channel(
        &self,
        input: GetBaiduChannelRequest,
    ) -> Result<GetBaiduChannelResponse, RusotoError<GetBaiduChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBaiduChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBaiduChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for a campaign.</p>
    async fn get_campaign(
        &self,
        input: GetCampaignRequest,
    ) -> Result<GetCampaignResponse, RusotoError<GetCampaignError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}",
            application_id = input.application_id,
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignError::from_response(response))
        }
    }

    /// <p>Retrieves information about all the activities for a campaign.</p>
    async fn get_campaign_activities(
        &self,
        input: GetCampaignActivitiesRequest,
    ) -> Result<GetCampaignActivitiesResponse, RusotoError<GetCampaignActivitiesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCampaignActivitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignActivitiesError::from_response(response))
        }
    }

    /// <p>Retrieves (queries) pre-aggregated data for a standard metric that applies to a campaign.</p>
    async fn get_campaign_date_range_kpi(
        &self,
        input: GetCampaignDateRangeKpiRequest,
    ) -> Result<GetCampaignDateRangeKpiResponse, RusotoError<GetCampaignDateRangeKpiError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/kpis/daterange/{kpi_name}",
            application_id = input.application_id,
            campaign_id = input.campaign_id,
            kpi_name = input.kpi_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("end-time", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("start-time", x);
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
                .deserialize::<GetCampaignDateRangeKpiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignDateRangeKpiError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for a specific version of a campaign.</p>
    async fn get_campaign_version(
        &self,
        input: GetCampaignVersionRequest,
    ) -> Result<GetCampaignVersionResponse, RusotoError<GetCampaignVersionError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/campaigns/{campaign_id}/versions/{version}",
            application_id = input.application_id,
            campaign_id = input.campaign_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCampaignVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignVersionError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for all versions of a campaign.</p>
    async fn get_campaign_versions(
        &self,
        input: GetCampaignVersionsRequest,
    ) -> Result<GetCampaignVersionsResponse, RusotoError<GetCampaignVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCampaignVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignVersionsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for all the campaigns that are associated with an application.</p>
    async fn get_campaigns(
        &self,
        input: GetCampaignsRequest,
    ) -> Result<GetCampaignsResponse, RusotoError<GetCampaignsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCampaignsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCampaignsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the history and status of each channel for an application.</p>
    async fn get_channels(
        &self,
        input: GetChannelsRequest,
    ) -> Result<GetChannelsResponse, RusotoError<GetChannelsError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetChannelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetChannelsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the email channel for an application.</p>
    async fn get_email_channel(
        &self,
        input: GetEmailChannelRequest,
    ) -> Result<GetEmailChannelResponse, RusotoError<GetEmailChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEmailChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailChannelError::from_response(response))
        }
    }

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the email channel.</p>
    async fn get_email_template(
        &self,
        input: GetEmailTemplateRequest,
    ) -> Result<GetEmailTemplateResponse, RusotoError<GetEmailTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/email",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
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
                .deserialize::<GetEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves information about the settings and attributes of a specific endpoint for an application.</p>
    async fn get_endpoint(
        &self,
        input: GetEndpointRequest,
    ) -> Result<GetEndpointResponse, RusotoError<GetEndpointError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints/{endpoint_id}",
            application_id = input.application_id,
            endpoint_id = input.endpoint_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEndpointError::from_response(response))
        }
    }

    /// <p>Retrieves information about the event stream settings for an application.</p>
    async fn get_event_stream(
        &self,
        input: GetEventStreamRequest,
    ) -> Result<GetEventStreamResponse, RusotoError<GetEventStreamError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEventStreamResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEventStreamError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of a specific export job for an application.</p>
    async fn get_export_job(
        &self,
        input: GetExportJobRequest,
    ) -> Result<GetExportJobResponse, RusotoError<GetExportJobError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/export/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetExportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetExportJobError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of all the export jobs for an application.</p>
    async fn get_export_jobs(
        &self,
        input: GetExportJobsRequest,
    ) -> Result<GetExportJobsResponse, RusotoError<GetExportJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetExportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetExportJobsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the GCM channel for an application.</p>
    async fn get_gcm_channel(
        &self,
        input: GetGcmChannelRequest,
    ) -> Result<GetGcmChannelResponse, RusotoError<GetGcmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetGcmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetGcmChannelError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of a specific import job for an application.</p>
    async fn get_import_job(
        &self,
        input: GetImportJobRequest,
    ) -> Result<GetImportJobResponse, RusotoError<GetImportJobError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/jobs/import/{job_id}",
            application_id = input.application_id,
            job_id = input.job_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImportJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImportJobError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of all the import jobs for an application.</p>
    async fn get_import_jobs(
        &self,
        input: GetImportJobsRequest,
    ) -> Result<GetImportJobsResponse, RusotoError<GetImportJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetImportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetImportJobsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for a journey.</p>
    async fn get_journey(
        &self,
        input: GetJourneyRequest,
    ) -> Result<GetJourneyResponse, RusotoError<GetJourneyError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}",
            application_id = input.application_id,
            journey_id = input.journey_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetJourneyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJourneyError::from_response(response))
        }
    }

    /// <p>Retrieves (queries) pre-aggregated data for a standard engagement metric that applies to a journey.</p>
    async fn get_journey_date_range_kpi(
        &self,
        input: GetJourneyDateRangeKpiRequest,
    ) -> Result<GetJourneyDateRangeKpiResponse, RusotoError<GetJourneyDateRangeKpiError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}/kpis/daterange/{kpi_name}",
            application_id = input.application_id,
            journey_id = input.journey_id,
            kpi_name = input.kpi_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("end-time", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("start-time", x);
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
                .deserialize::<GetJourneyDateRangeKpiResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJourneyDateRangeKpiError::from_response(response))
        }
    }

    /// <p>Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey activity.</p>
    async fn get_journey_execution_activity_metrics(
        &self,
        input: GetJourneyExecutionActivityMetricsRequest,
    ) -> Result<
        GetJourneyExecutionActivityMetricsResponse,
        RusotoError<GetJourneyExecutionActivityMetricsError>,
    > {
        let request_uri = format!("/v1/apps/{application_id}/journeys/{journey_id}/activities/{journey-activity-id}/execution-metrics", application_id = input.application_id, journey_activity_id = input.journey_activity_id, journey_id = input.journey_id);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
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
                .deserialize::<GetJourneyExecutionActivityMetricsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJourneyExecutionActivityMetricsError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey.</p>
    async fn get_journey_execution_metrics(
        &self,
        input: GetJourneyExecutionMetricsRequest,
    ) -> Result<GetJourneyExecutionMetricsResponse, RusotoError<GetJourneyExecutionMetricsError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}/execution-metrics",
            application_id = input.application_id,
            journey_id = input.journey_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
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
                .deserialize::<GetJourneyExecutionMetricsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJourneyExecutionMetricsError::from_response(response))
        }
    }

    /// <p>Retrieves the content and settings of a message template for messages that are sent through a push notification channel.</p>
    async fn get_push_template(
        &self,
        input: GetPushTemplateRequest,
    ) -> Result<GetPushTemplateResponse, RusotoError<GetPushTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/push",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
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
                .deserialize::<GetPushTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPushTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves information about the configuration, dimension, and other settings for a specific segment that's associated with an application.</p>
    async fn get_segment(
        &self,
        input: GetSegmentRequest,
    ) -> Result<GetSegmentResponse, RusotoError<GetSegmentError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}",
            application_id = input.application_id,
            segment_id = input.segment_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the export jobs for a segment.</p>
    async fn get_segment_export_jobs(
        &self,
        input: GetSegmentExportJobsRequest,
    ) -> Result<GetSegmentExportJobsResponse, RusotoError<GetSegmentExportJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentExportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentExportJobsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the import jobs for a segment.</p>
    async fn get_segment_import_jobs(
        &self,
        input: GetSegmentImportJobsRequest,
    ) -> Result<GetSegmentImportJobsResponse, RusotoError<GetSegmentImportJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentImportJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentImportJobsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the configuration, dimension, and other settings for a specific version of a segment that's associated with an application.</p>
    async fn get_segment_version(
        &self,
        input: GetSegmentVersionRequest,
    ) -> Result<GetSegmentVersionResponse, RusotoError<GetSegmentVersionError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/segments/{segment_id}/versions/{version}",
            application_id = input.application_id,
            segment_id = input.segment_id,
            version = input.version
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentVersionError::from_response(response))
        }
    }

    /// <p>Retrieves information about the configuration, dimension, and other settings for all the versions of a specific segment that's associated with an application.</p>
    async fn get_segment_versions(
        &self,
        input: GetSegmentVersionsRequest,
    ) -> Result<GetSegmentVersionsResponse, RusotoError<GetSegmentVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentVersionsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the configuration, dimension, and other settings for all the segments that are associated with an application.</p>
    async fn get_segments(
        &self,
        input: GetSegmentsRequest,
    ) -> Result<GetSegmentsResponse, RusotoError<GetSegmentsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSegmentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSegmentsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the SMS channel for an application.</p>
    async fn get_sms_channel(
        &self,
        input: GetSmsChannelRequest,
    ) -> Result<GetSmsChannelResponse, RusotoError<GetSmsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSmsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSmsChannelError::from_response(response))
        }
    }

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the SMS channel.</p>
    async fn get_sms_template(
        &self,
        input: GetSmsTemplateRequest,
    ) -> Result<GetSmsTemplateResponse, RusotoError<GetSmsTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/sms",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
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
                .deserialize::<GetSmsTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSmsTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves information about all the endpoints that are associated with a specific user ID.</p>
    async fn get_user_endpoints(
        &self,
        input: GetUserEndpointsRequest,
    ) -> Result<GetUserEndpointsResponse, RusotoError<GetUserEndpointsError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users/{user_id}",
            application_id = input.application_id,
            user_id = input.user_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetUserEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetUserEndpointsError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status and settings of the voice channel for an application.</p>
    async fn get_voice_channel(
        &self,
        input: GetVoiceChannelRequest,
    ) -> Result<GetVoiceChannelResponse, RusotoError<GetVoiceChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetVoiceChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceChannelError::from_response(response))
        }
    }

    /// <p>Retrieves the content and settings of a message template for messages that are sent through the voice channel.</p>
    async fn get_voice_template(
        &self,
        input: GetVoiceTemplateRequest,
    ) -> Result<GetVoiceTemplateResponse, RusotoError<GetVoiceTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/voice",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.version {
            params.put("version", x);
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
                .deserialize::<GetVoiceTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetVoiceTemplateError::from_response(response))
        }
    }

    /// <p>Retrieves information about the status, configuration, and other settings for all the journeys that are associated with an application.</p>
    async fn list_journeys(
        &self,
        input: ListJourneysRequest,
    ) -> Result<ListJourneysResponse, RusotoError<ListJourneysError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys",
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJourneysResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJourneysError::from_response(response))
        }
    }

    /// <p>Retrieves all the tags (keys and values) that are associated with an application, campaign, journey, message template, or segment.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Retrieves information about all the versions of a specific message template.</p>
    async fn list_template_versions(
        &self,
        input: ListTemplateVersionsRequest,
    ) -> Result<ListTemplateVersionsResponse, RusotoError<ListTemplateVersionsError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/{template_type}/versions",
            template_name = input.template_name,
            template_type = input.template_type
        );

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
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
                .deserialize::<ListTemplateVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplateVersionsError::from_response(response))
        }
    }

    /// <p>Retrieves information about all the message templates that are associated with your Amazon Pinpoint account.</p>
    async fn list_templates(
        &self,
        input: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse, RusotoError<ListTemplatesError>> {
        let request_uri = "/v1/templates";

        let mut request = SignedRequest::new("GET", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("next-token", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("page-size", x);
        }
        if let Some(ref x) = input.prefix {
            params.put("prefix", x);
        }
        if let Some(ref x) = input.template_type {
            params.put("template-type", x);
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
                .deserialize::<ListTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTemplatesError::from_response(response))
        }
    }

    /// <p>Retrieves information about a phone number.</p>
    async fn phone_number_validate(
        &self,
        input: PhoneNumberValidateRequest,
    ) -> Result<PhoneNumberValidateResponse, RusotoError<PhoneNumberValidateError>> {
        let request_uri = "/v1/phone/number/validate";

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.number_validate_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PhoneNumberValidateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PhoneNumberValidateError::from_response(response))
        }
    }

    /// <p>Creates a new event stream for an application or updates the settings of an existing event stream for an application.</p>
    async fn put_event_stream(
        &self,
        input: PutEventStreamRequest,
    ) -> Result<PutEventStreamResponse, RusotoError<PutEventStreamError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/eventstream",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_event_stream).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEventStreamResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventStreamError::from_response(response))
        }
    }

    /// <p>Creates a new event to record for endpoints, or creates or updates endpoint data that existing events are associated with.</p>
    async fn put_events(
        &self,
        input: PutEventsRequest,
    ) -> Result<PutEventsResponse, RusotoError<PutEventsError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/events",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.events_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEventsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventsError::from_response(response))
        }
    }

    /// <p>Removes one or more attributes, of the same attribute type, from all the endpoints that are associated with an application.</p>
    async fn remove_attributes(
        &self,
        input: RemoveAttributesRequest,
    ) -> Result<RemoveAttributesResponse, RusotoError<RemoveAttributesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RemoveAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveAttributesError::from_response(response))
        }
    }

    /// <p>Creates and sends a direct message.</p>
    async fn send_messages(
        &self,
        input: SendMessagesRequest,
    ) -> Result<SendMessagesResponse, RusotoError<SendMessagesError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.message_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendMessagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendMessagesError::from_response(response))
        }
    }

    /// <p>Creates and sends a message to a list of users.</p>
    async fn send_users_messages(
        &self,
        input: SendUsersMessagesRequest,
    ) -> Result<SendUsersMessagesResponse, RusotoError<SendUsersMessagesError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/users-messages",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.send_users_message_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendUsersMessagesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendUsersMessagesError::from_response(response))
        }
    }

    /// <p>Adds one or more tags (keys and values) to an application, campaign, journey, message template, or segment.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.tags_model).unwrap());
        request.set_payload(encoded);

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
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags (keys and values) from an application, campaign, journey, message template, or segment.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

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
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Enables the ADM channel for an application or updates the status and settings of the ADM channel for an application.</p>
    async fn update_adm_channel(
        &self,
        input: UpdateAdmChannelRequest,
    ) -> Result<UpdateAdmChannelResponse, RusotoError<UpdateAdmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/adm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.adm_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateAdmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAdmChannelError::from_response(response))
        }
    }

    /// <p>Enables the APNs channel for an application or updates the status and settings of the APNs channel for an application.</p>
    async fn update_apns_channel(
        &self,
        input: UpdateApnsChannelRequest,
    ) -> Result<UpdateApnsChannelResponse, RusotoError<UpdateApnsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApnsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApnsChannelError::from_response(response))
        }
    }

    /// <p>Enables the APNs sandbox channel for an application or updates the status and settings of the APNs sandbox channel for an application.</p>
    async fn update_apns_sandbox_channel(
        &self,
        input: UpdateApnsSandboxChannelRequest,
    ) -> Result<UpdateApnsSandboxChannelResponse, RusotoError<UpdateApnsSandboxChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApnsSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApnsSandboxChannelError::from_response(response))
        }
    }

    /// <p>Enables the APNs VoIP channel for an application or updates the status and settings of the APNs VoIP channel for an application.</p>
    async fn update_apns_voip_channel(
        &self,
        input: UpdateApnsVoipChannelRequest,
    ) -> Result<UpdateApnsVoipChannelResponse, RusotoError<UpdateApnsVoipChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApnsVoipChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApnsVoipChannelError::from_response(response))
        }
    }

    /// <p>Enables the APNs VoIP sandbox channel for an application or updates the status and settings of the APNs VoIP sandbox channel for an application.</p>
    async fn update_apns_voip_sandbox_channel(
        &self,
        input: UpdateApnsVoipSandboxChannelRequest,
    ) -> Result<UpdateApnsVoipSandboxChannelResponse, RusotoError<UpdateApnsVoipSandboxChannelError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/apns_voip_sandbox",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.apns_voip_sandbox_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApnsVoipSandboxChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApnsVoipSandboxChannelError::from_response(response))
        }
    }

    /// <p>Updates the settings for an application.</p>
    async fn update_application_settings(
        &self,
        input: UpdateApplicationSettingsRequest,
    ) -> Result<UpdateApplicationSettingsResponse, RusotoError<UpdateApplicationSettingsError>>
    {
        let request_uri = format!(
            "/v1/apps/{application_id}/settings",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_application_settings_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApplicationSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApplicationSettingsError::from_response(response))
        }
    }

    /// <p>Enables the Baidu channel for an application or updates the status and settings of the Baidu channel for an application.</p>
    async fn update_baidu_channel(
        &self,
        input: UpdateBaiduChannelRequest,
    ) -> Result<UpdateBaiduChannelResponse, RusotoError<UpdateBaiduChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/baidu",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.baidu_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateBaiduChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateBaiduChannelError::from_response(response))
        }
    }

    /// <p>Updates the configuration and other settings for a campaign.</p>
    async fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> Result<UpdateCampaignResponse, RusotoError<UpdateCampaignError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCampaignError::from_response(response))
        }
    }

    /// <p>Enables the email channel for an application or updates the status and settings of the email channel for an application.</p>
    async fn update_email_channel(
        &self,
        input: UpdateEmailChannelRequest,
    ) -> Result<UpdateEmailChannelResponse, RusotoError<UpdateEmailChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/email",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEmailChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEmailChannelError::from_response(response))
        }
    }

    /// <p>Updates an existing message template for messages that are sent through the email channel.</p>
    async fn update_email_template(
        &self,
        input: UpdateEmailTemplateRequest,
    ) -> Result<UpdateEmailTemplateResponse, RusotoError<UpdateEmailTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/email",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.email_template_request).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.create_new_version {
            params.put("create-new-version", x);
        }
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEmailTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEmailTemplateError::from_response(response))
        }
    }

    /// <p>Creates a new endpoint for an application or updates the settings and attributes of an existing endpoint for an application. You can also use this operation to define custom attributes (Attributes, Metrics, and UserAttributes properties) for an endpoint.</p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> Result<UpdateEndpointResponse, RusotoError<UpdateEndpointError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEndpointError::from_response(response))
        }
    }

    /// <p><p>Creates a new batch of endpoints for an application or updates the settings and attributes of a batch of existing endpoints for an application. You can also use this operation to define custom attributes (Attributes, Metrics, and UserAttributes properties) for a batch of endpoints.</p></p>
    async fn update_endpoints_batch(
        &self,
        input: UpdateEndpointsBatchRequest,
    ) -> Result<UpdateEndpointsBatchResponse, RusotoError<UpdateEndpointsBatchError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/endpoints",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.endpoint_batch_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateEndpointsBatchResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEndpointsBatchError::from_response(response))
        }
    }

    /// <p>Enables the GCM channel for an application or updates the status and settings of the GCM channel for an application.</p>
    async fn update_gcm_channel(
        &self,
        input: UpdateGcmChannelRequest,
    ) -> Result<UpdateGcmChannelResponse, RusotoError<UpdateGcmChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/gcm",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.gcm_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateGcmChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateGcmChannelError::from_response(response))
        }
    }

    /// <p>Updates the configuration and other settings for a journey.</p>
    async fn update_journey(
        &self,
        input: UpdateJourneyRequest,
    ) -> Result<UpdateJourneyResponse, RusotoError<UpdateJourneyError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}",
            application_id = input.application_id,
            journey_id = input.journey_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.write_journey_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateJourneyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJourneyError::from_response(response))
        }
    }

    /// <p>Cancels (stops) an active journey.</p>
    async fn update_journey_state(
        &self,
        input: UpdateJourneyStateRequest,
    ) -> Result<UpdateJourneyStateResponse, RusotoError<UpdateJourneyStateError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/journeys/{journey_id}/state",
            application_id = input.application_id,
            journey_id = input.journey_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.journey_state_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateJourneyStateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJourneyStateError::from_response(response))
        }
    }

    /// <p>Updates an existing message template for messages that are sent through a push notification channel.</p>
    async fn update_push_template(
        &self,
        input: UpdatePushTemplateRequest,
    ) -> Result<UpdatePushTemplateResponse, RusotoError<UpdatePushTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/push",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.push_notification_template_request).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.create_new_version {
            params.put("create-new-version", x);
        }
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdatePushTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePushTemplateError::from_response(response))
        }
    }

    /// <p>Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application.</p>
    async fn update_segment(
        &self,
        input: UpdateSegmentRequest,
    ) -> Result<UpdateSegmentResponse, RusotoError<UpdateSegmentError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSegmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSegmentError::from_response(response))
        }
    }

    /// <p>Enables the SMS channel for an application or updates the status and settings of the SMS channel for an application.</p>
    async fn update_sms_channel(
        &self,
        input: UpdateSmsChannelRequest,
    ) -> Result<UpdateSmsChannelResponse, RusotoError<UpdateSmsChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/sms",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSmsChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSmsChannelError::from_response(response))
        }
    }

    /// <p>Updates an existing message template for messages that are sent through the SMS channel.</p>
    async fn update_sms_template(
        &self,
        input: UpdateSmsTemplateRequest,
    ) -> Result<UpdateSmsTemplateResponse, RusotoError<UpdateSmsTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/sms",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.sms_template_request).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.create_new_version {
            params.put("create-new-version", x);
        }
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateSmsTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateSmsTemplateError::from_response(response))
        }
    }

    /// <p>Changes the status of a specific version of a message template to <i>active</i>.</p>
    async fn update_template_active_version(
        &self,
        input: UpdateTemplateActiveVersionRequest,
    ) -> Result<UpdateTemplateActiveVersionResponse, RusotoError<UpdateTemplateActiveVersionError>>
    {
        let request_uri = format!(
            "/v1/templates/{template_name}/{template_type}/active-version",
            template_name = input.template_name,
            template_type = input.template_type
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.template_active_version_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateTemplateActiveVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTemplateActiveVersionError::from_response(response))
        }
    }

    /// <p>Enables the voice channel for an application or updates the status and settings of the voice channel for an application.</p>
    async fn update_voice_channel(
        &self,
        input: UpdateVoiceChannelRequest,
    ) -> Result<UpdateVoiceChannelResponse, RusotoError<UpdateVoiceChannelError>> {
        let request_uri = format!(
            "/v1/apps/{application_id}/channels/voice",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.voice_channel_request).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVoiceChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVoiceChannelError::from_response(response))
        }
    }

    /// <p>Updates an existing message template for messages that are sent through the voice channel.</p>
    async fn update_voice_template(
        &self,
        input: UpdateVoiceTemplateRequest,
    ) -> Result<UpdateVoiceTemplateResponse, RusotoError<UpdateVoiceTemplateError>> {
        let request_uri = format!(
            "/v1/templates/{template_name}/voice",
            template_name = input.template_name
        );

        let mut request = SignedRequest::new("PUT", "mobiletargeting", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input.voice_template_request).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.create_new_version {
            params.put("create-new-version", x);
        }
        if let Some(ref x) = input.version {
            params.put("version", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVoiceTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVoiceTemplateError::from_response(response))
        }
    }
}
