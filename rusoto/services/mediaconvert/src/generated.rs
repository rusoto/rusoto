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

#[allow(unused_imports)]
use rusoto_core::signature::decode_uri;

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AAC. The service accepts one of two mutually exclusive groups of AAC settings--VBR and CBR. To select one of these modes, set the value of Bitrate control mode (rateControlMode) to &quot;VBR&quot; or &quot;CBR&quot;.  In VBR mode, you control the audio quality with the setting VBR quality (vbrQuality). In CBR mode, you use the setting Bitrate (bitrate). Defaults and valid values depend on the rate control mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AacSettings {
    #[serde(rename = "AudioDescriptionBroadcasterMix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_broadcaster_mix: Option<String>,
    /// <p>Average bitrate in bits/second. Defaults and valid values depend on rate control mode and profile.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    #[serde(rename = "CodingMode")]
    pub coding_mode: String,
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "RawFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<String>,
    /// <p>Sample rate in Hz. Valid values depend on rate control mode and profile.</p>
    #[serde(rename = "SampleRate")]
    pub sample_rate: i64,
    #[serde(rename = "Specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    #[serde(rename = "VbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AC3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ac3Settings {
    /// <p>Average bitrate in bits/second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital, dialnorm will be passed through.</p>
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    #[serde(rename = "DynamicRangeCompressionProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_profile: Option<String>,
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>Sample rate in hz. Sample rate is always 48000.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AIFF.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiffSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "BitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Set Channels to specify the number of channels in this output audio track. Choosing Mono in the console will give you 1 output channel; choosing Stereo will give you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Settings for ancillary captions source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AncillarySourceSettings {
    /// <p>Specifies the 608 channel number in the ancillary data track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "SourceAncillaryChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i64>,
}

/// <p>Audio codec settings (CodecSettings) under (AudioDescriptions) contains the group of settings related to audio encoding. The settings in this group vary depending on the value you choose for Audio codec (Codec). For each codec enum you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * AAC, AacSettings * MP2, Mp2Settings * WAV, WavSettings * AIFF, AiffSettings * AC3, Ac3Settings * EAC3, Eac3Settings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioCodecSettings {
    #[serde(rename = "AacSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,
    #[serde(rename = "Ac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_3_settings: Option<Ac3Settings>,
    #[serde(rename = "AiffSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aiff_settings: Option<AiffSettings>,
    #[serde(rename = "Codec")]
    pub codec: String,
    #[serde(rename = "Eac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_settings: Option<Eac3Settings>,
    #[serde(rename = "Mp2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_2_settings: Option<Mp2Settings>,
    #[serde(rename = "WavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

/// <p>Description of audio output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioDescription {
    #[serde(rename = "AudioNormalizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,
    /// <p>Specifies which audio data to use from each input. In the simplest case, specify an &quot;Audio Selector&quot;:#inputs-audio<em>selector by name based on its order within each input. For example if you specify &quot;Audio Selector 3&quot;, then the third audio selector will be used from each input. If an input does not have an &quot;Audio Selector 3&quot;, then the audio selector marked as &quot;default&quot; in that input will be used. If there is no audio selector marked as &quot;default&quot;, silence will be inserted for the duration of that input. Alternatively, an &quot;Audio Selector Group&quot;:#inputs-audio</em>selector<em>group name may be specified, with similar default/silence behavior. If no audio</em>source_name is specified, then &quot;Audio Selector 1&quot; will be chosen automatically.</p>
    #[serde(rename = "AudioSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_source_name: Option<String>,
    /// <p>Applies only if Follow Input Audio Type is unchecked (false). A number between 0 and 255. The following are defined in ISO-IEC 13818-1: 0 = Undefined, 1 = Clean Effects, 2 = Hearing Impaired, 3 = Visually Impaired Commentary, 4-255 = Reserved.</p>
    #[serde(rename = "AudioType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<i64>,
    #[serde(rename = "AudioTypeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<String>,
    #[serde(rename = "CodecSettings")]
    pub codec_settings: AudioCodecSettings,
    /// <p>Specify the language for this audio output track, using the ISO 639-2 or ISO 639-3 three-letter language code. The language specified will be used when &#39;Follow Input Language Code&#39; is not selected or when &#39;Follow Input Language Code&#39; is selected but there is no ISO 639 language code specified by the input.</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Indicates the language of the audio output track. The ISO 639 language specified in the &#39;Language Code&#39; drop down will be used when &#39;Follow Input Language Code&#39; is not selected or when &#39;Follow Input Language Code&#39; is selected but there is no ISO 639 language code specified by the input.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageCodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<String>,
    /// <p>Advanced audio remixing settings.</p>
    #[serde(rename = "RemixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    /// <p>Used for MS Smooth and Apple HLS outputs. Indicates the name displayed by the player (eg. English, or Director Commentary). Alphanumeric characters, spaces, and underscore are legal.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Advanced audio normalization settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNormalizationSettings {
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "AlgorithmControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<String>,
    /// <p>Content measuring above this level will be corrected to the target level. Content measuring below this level will not be corrected. Gating only applies when not using real<em>time</em>correction.</p>
    #[serde(rename = "CorrectionGateLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_gate_level: Option<i64>,
    #[serde(rename = "LoudnessLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness_logging: Option<String>,
    #[serde(rename = "PeakCalculation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_calculation: Option<String>,
    /// <p>Target LKFS(loudness) to adjust volume to. If no value is entered, a default value will be used according to the chosen algorithm. The CALM Act (1770-1) recommends a target of -24 LKFS. The EBU R-128 specification (1770-2) recommends a target of -23 LKFS.</p>
    #[serde(rename = "TargetLkfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lkfs: Option<f64>,
}

/// <p>Selector for Audio</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioSelector {
    /// <p>Selects a specific language code from within an audio source, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    #[serde(rename = "DefaultSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_selection: Option<String>,
    /// <p>Specifies audio data from an external file source.</p>
    #[serde(rename = "ExternalAudioFileInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_audio_file_input: Option<String>,
    /// <p>Selects a specific language code from within an audio source.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specifies a time delta in milliseconds to offset the audio from the input video.</p>
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// <p>Selects a specific PID from within an audio source (e.g. 257 selects PID 0x101).</p>
    #[serde(rename = "Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<Vec<i64>>,
    /// <p>Use this setting for input streams that contain Dolby E, to have the service extract specific program data from the track. To select multiple programs, create multiple selectors with the same Track and different Program numbers. In the console, this setting is visible when you set Selector type to Track. Choose the program number from the dropdown list. If you are sending a JSON file, provide the program ID, which is part of the audio metadata. If your input file has incorrect metadata, you can choose All channels instead of a program number to have the service ignore the program IDs and include all the programs in the track.</p>
    #[serde(rename = "ProgramSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_selection: Option<i64>,
    /// <p>Use these settings to reorder the audio channels of one input to match those of another input. This allows you to combine the two files into a single output, one after the other.</p>
    #[serde(rename = "RemixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    #[serde(rename = "SelectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_type: Option<String>,
    /// <p>Identify a track from the input audio to include in this selector by entering the track index number. To include several tracks in a single audio selector, specify multiple tracks as follows. Using the console, enter a comma-separated list. For examle, type &quot;1,2,3&quot; to include tracks 1 through 3. Specifying directly in your JSON job file, provide the track numbers in an array. For example, &quot;tracks&quot;: [1,2,3].</p>
    #[serde(rename = "Tracks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<i64>>,
}

/// <p>Group of Audio Selectors</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioSelectorGroup {
    /// <p>Name of an Audio Selector within the same input to include in the group.  Audio selector names are standardized, based on their order within the input (e.g., &quot;Audio Selector 1&quot;). The audio selector name parameter can be repeated to add any number of audio selectors to the group.</p>
    #[serde(rename = "AudioSelectorNames")]
    pub audio_selector_names: Vec<String>,
}

/// <p>Settings for Avail Blanking</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailBlanking {
    /// <p>Blanking image to be used. Leave empty for solid black. Only bmp and png images are supported.</p>
    #[serde(rename = "AvailBlankingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking_image: Option<String>,
}

/// <p>Burn-In Destination Settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurninDestinationSettings {
    #[serde(rename = "Alignment")]
    pub alignment: String,
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontOpacity")]
    pub font_opacity: i64,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    #[serde(rename = "OutlineColor")]
    pub outline_color: String,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineSize")]
    pub outline_size: i64,
    #[serde(rename = "ShadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    #[serde(rename = "TeletextSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_spacing: Option<String>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit x_position is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "XPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit y_position is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "YPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelJobRequest {
    /// <p>The Job ID of the job to be cancelled.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>Description of Caption output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDescription {
    /// <p>Specifies which &quot;Caption Selector&quot;:#inputs-caption_selector to use from each input when generating captions. The name should be of the format &quot;Caption Selector <N>&quot;, which denotes that the Nth Caption Selector will be used from each input.</p>
    #[serde(rename = "CaptionSelectorName")]
    pub caption_selector_name: String,
    /// <p>Indicates the language of the caption output track, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    #[serde(rename = "DestinationSettings")]
    pub destination_settings: CaptionDestinationSettings,
    /// <p>Indicates the language of the caption output track.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Human readable information to indicate captions available for players (eg. English, or Spanish). Alphanumeric characters, spaces, and underscore are legal.</p>
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Caption Description for preset</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDescriptionPreset {
    /// <p>Indicates the language of the caption output track, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    #[serde(rename = "DestinationSettings")]
    pub destination_settings: CaptionDestinationSettings,
    /// <p>Indicates the language of the caption output track.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Human readable information to indicate captions available for players (eg. English, or Spanish). Alphanumeric characters, spaces, and underscore are legal.</p>
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Specific settings required by destination type. Note that burnin<em>destination</em>settings are not available if the source of the caption data is Embedded or Teletext.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDestinationSettings {
    #[serde(rename = "BurninDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnin_destination_settings: Option<BurninDestinationSettings>,
    #[serde(rename = "DestinationType")]
    pub destination_type: String,
    #[serde(rename = "DvbSubDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    #[serde(rename = "SccDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_destination_settings: Option<SccDestinationSettings>,
    #[serde(rename = "TeletextDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    #[serde(rename = "TtmlDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,
}

/// <p>Set up captions in your outputs by first selecting them from your input here.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionSelector {
    /// <p>The specific language to extract from source, using the ISO 639-2 or ISO 639-3 three-letter language code. If input is SCTE-27, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub and output is Burn-in or SMPTE-TT, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub that is being passed through, omit this field (and PID field); there is no way to extract a specific language with pass-through captions.</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>The specific language to extract from source. If input is SCTE-27, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub and output is Burn-in or SMPTE-TT, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub that is being passed through, omit this field (and PID field); there is no way to extract a specific language with pass-through captions.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "SourceSettings")]
    pub source_settings: CaptionSourceSettings,
}

/// <p>Source settings (SourceSettings) contains the group of settings for captions in the input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionSourceSettings {
    #[serde(rename = "AncillarySourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,
    #[serde(rename = "DvbSubSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    #[serde(rename = "EmbeddedSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    #[serde(rename = "FileSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_source_settings: Option<FileSourceSettings>,
    #[serde(rename = "SourceType")]
    pub source_type: String,
    #[serde(rename = "TeletextSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
}

/// <p>Channel mapping (ChannelMapping) contains the group of fields that hold the remixing value for each channel. Units are in dB. Acceptable values are within the range from -60 (mute) through 6. A setting of 0 passes the input channel unchanged to the output channel (no attenuation or amplification).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelMapping {
    /// <p>List of output channels</p>
    #[serde(rename = "OutputChannels")]
    pub output_channels: Vec<OutputChannelMapping>,
}

/// <p>Settings for CMAF encryption</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafEncryptionSettings {
    /// <p>This is a 128-bit, 16-byte hex value represented by a 32-character text string. If this parameter is not set then the Initialization Vector will follow the segment number by default.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    #[serde(rename = "InitializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    #[serde(rename = "StaticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to CMAF<em>GROUP</em>SETTINGS. Each output in a CMAF Output Group may only contain a single video, audio, or caption output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafGroupSettings {
    /// <p>A partial URI prefix that will be put in the manifest file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "ClientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    #[serde(rename = "CodecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "FragmentLength")]
    pub fragment_length: i64,
    #[serde(rename = "ManifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    #[serde(rename = "ManifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "MinBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Use this setting to specify the length, in seconds, of each individual CMAF segment. This value applies to the whole package; that is, to every output in the output group. Note that segments end on the first keyframe after this number of seconds, so the actual segment length might be slightly longer. If you set Segment control (CmafSegmentControl) to single file, the service puts the content of each output in a single file that has metadata that marks these segments. If you set it to segmented files, the service creates multiple files for each output, each with the content of one segment.</p>
    #[serde(rename = "SegmentLength")]
    pub segment_length: i64,
    #[serde(rename = "StreamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    #[serde(rename = "WriteDashManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_dash_manifest: Option<String>,
    #[serde(rename = "WriteHlsManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_hls_manifest: Option<String>,
}

/// <p>Settings for color correction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorCorrector {
    /// <p>Brightness level.</p>
    #[serde(rename = "Brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i64>,
    #[serde(rename = "ColorSpaceConversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_conversion: Option<String>,
    /// <p>Contrast level.</p>
    #[serde(rename = "Contrast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<i64>,
    #[serde(rename = "Hdr10Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_metadata: Option<Hdr10Metadata>,
    /// <p>Hue in degrees.</p>
    #[serde(rename = "Hue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<i64>,
    /// <p>Saturation level.</p>
    #[serde(rename = "Saturation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation: Option<i64>,
}

/// <p>Container specific settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerSettings {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "F4vSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_4v_settings: Option<F4vSettings>,
    #[serde(rename = "M2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_2ts_settings: Option<M2tsSettings>,
    #[serde(rename = "M3u8Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_3u_8_settings: Option<M3u8Settings>,
    #[serde(rename = "MovSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mov_settings: Option<MovSettings>,
    #[serde(rename = "Mp4Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_settings: Option<Mp4Settings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// <p>Idempotency token for CreateJob operation.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>When you create a job, you can either specify a job template or specify the transcoding settings individually</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>Optional. When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.</p>
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Settings")]
    pub settings: JobSettings,
    /// <p>User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.</p>
    #[serde(rename = "UserMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateJobResponse {
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobTemplateRequest {
    /// <p>Optional. A category for the job template you are creating</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. A description of the job template you are creating.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the job template you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: JobTemplateSettings,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateJobTemplateResponse {
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePresetRequest {
    /// <p>Optional. A category for the preset you are creating.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. A description of the preset you are creating.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the preset you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settings")]
    pub settings: PresetSettings,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreatePresetResponse {
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateQueueRequest {
    /// <p>Optional. A description of the queue you are creating.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateQueueResponse {
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Specifies DRM settings for DASH outputs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashIsoEncryptionSettings {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to DASH<em>ISO</em>GROUP_SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashIsoGroupSettings {
    /// <p>A partial URI prefix that will be put in the manifest (.mpd) file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashIsoEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "FragmentLength")]
    pub fragment_length: i64,
    #[serde(rename = "HbbtvCompliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hbbtv_compliance: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "MinBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of mpd segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer. When Emit Single File is checked, the segmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "SegmentLength")]
    pub segment_length: i64,
}

/// <p>Settings for deinterlacer</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deinterlacer {
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "Control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteJobTemplateRequest {
    /// <p>The name of the job template to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteJobTemplateResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePresetRequest {
    /// <p>The name of the preset to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeletePresetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteQueueRequest {
    /// <p>The name of the queue to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteQueueResponse {}

/// <p>DescribeEndpointsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointsRequest {
    /// <p>Optional. Max number of endpoints, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of endpoints.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>List of endpoints</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// <p>Use this string to request the next batch of endpoints.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Inserts DVB Network Information Table (NIT) at the specified table repetition interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbNitSettings {
    /// <p>The numeric value placed in the Network Information Table (NIT).</p>
    #[serde(rename = "NetworkId")]
    pub network_id: i64,
    /// <p>The network name text placed in the network<em>name</em>descriptor inside the Network Information Table. Maximum length is 256 characters.</p>
    #[serde(rename = "NetworkName")]
    pub network_name: String,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "NitInterval")]
    pub nit_interval: i64,
}

/// <p>Inserts DVB Service Description Table (NIT) at the specified table repetition interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbSdtSettings {
    #[serde(rename = "OutputSdt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sdt: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "SdtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdt_interval: Option<i64>,
    /// <p>The service name placed in the service_descriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The service provider name placed in the service_descriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "ServiceProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
}

/// <p>DVB-Sub Destination Settings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbSubDestinationSettings {
    #[serde(rename = "Alignment")]
    pub alignment: String,
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontOpacity")]
    pub font_opacity: i64,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    #[serde(rename = "OutlineColor")]
    pub outline_color: String,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineSize")]
    pub outline_size: i64,
    #[serde(rename = "ShadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ShadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    #[serde(rename = "TeletextSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_spacing: Option<String>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit x_position is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "XPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit y_position is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "YPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

/// <p>DVB Sub Source Settings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbSubSourceSettings {
    /// <p>When using DVB-Sub with Burn-In or SMPTE-TT, use this PID for the source content. Unused for DVB-Sub passthrough. All DVB-Sub content is passed through, regardless of selectors.</p>
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

/// <p>Inserts DVB Time and Date Table (TDT) at the specified table repetition interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbTdtSettings {
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "TdtInterval")]
    pub tdt_interval: i64,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eac3Settings {
    #[serde(rename = "AttenuationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    /// <p>Average bitrate in bits/second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(rename = "DcFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital Plus, dialnorm will be passed through.</p>
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    #[serde(rename = "DynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    #[serde(rename = "DynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    #[serde(rename = "LfeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<String>,
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>Left only/Right only center mix level. Only used for 3/2 coding mode.
    /// Valid values: 3.0, 1.5, 0.0, -1.5 -3.0 -4.5 -6.0 -60</p>
    #[serde(rename = "LoRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Left only/Right only surround mix level. Only used for 3/2 coding mode.
    /// Valid values: -1.5 -3.0 -4.5 -6.0 -60</p>
    #[serde(rename = "LoRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Left total/Right total center mix level. Only used for 3/2 coding mode.
    /// Valid values: 3.0, 1.5, 0.0, -1.5 -3.0 -4.5 -6.0 -60</p>
    #[serde(rename = "LtRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Left total/Right total surround mix level. Only used for 3/2 coding mode.
    /// Valid values: -1.5 -3.0 -4.5 -6.0 -60</p>
    #[serde(rename = "LtRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    #[serde(rename = "PassthroughControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<String>,
    #[serde(rename = "PhaseControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<String>,
    /// <p>Sample rate in hz. Sample rate is always 48000.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    #[serde(rename = "StereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    #[serde(rename = "SurroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
    #[serde(rename = "SurroundMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_mode: Option<String>,
}

/// <p>Settings for embedded captions Source</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedSourceSettings {
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "Source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_channel_number: Option<i64>,
    /// <p>Specifies the video track index used for extracting captions. The system only supports one input video track, so this should always be set to &#39;1&#39;.</p>
    #[serde(rename = "Source608TrackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_track_number: Option<i64>,
}

/// <p>Describes account specific API endpoint</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Endpoint {
    /// <p>URL of endpoint</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExceptionBody {
    pub message: Option<String>,
}

/// <p>Settings for F4v container</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct F4vSettings {
    #[serde(rename = "MoovPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moov_placement: Option<String>,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to FILE<em>GROUP</em>SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileGroupSettings {
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// <p>Settings for File-based Captions in Source</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSourceSettings {
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>External caption file used for loading captions. Accepted file extensions are &#39;scc&#39;, &#39;ttml&#39;, &#39;dfxp&#39;, &#39;stl&#39;, &#39;srt&#39;, and &#39;smi&#39;.</p>
    #[serde(rename = "SourceFile")]
    pub source_file: String,
    /// <p>Specifies a time delta in seconds to offset the captions from the source file.</p>
    #[serde(rename = "TimeDelta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delta: Option<i64>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value FRAME_CAPTURE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameCaptureSettings {
    /// <p>Frame capture will encode the first frame of the output stream, then one frame every framerateDenominator/framerateNumerator seconds. For example, settings of framerateNumerator = 1 and framerateDenominator = 3 (a rate of 1/3 frame per second) will capture the first frame, then 1 frame every 3s. Files will be named as filename.n.jpg where n is the 0-based sequence number of each Capture.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Frame capture will encode the first frame of the output stream, then one frame every framerateDenominator/framerateNumerator seconds. For example, settings of framerateNumerator = 1 and framerateDenominator = 3 (a rate of 1/3 frame per second) will capture the first frame, then 1 frame every 3s. Files will be named as filename.NNNNNNN.jpg where N is the 0-based frame sequence number zero padded to 7 decimal places.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Maximum number of captures (encoded jpg output files).</p>
    #[serde(rename = "MaxCaptures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_captures: Option<i64>,
    /// <p>JPEG Quality - a higher value equals higher quality.</p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRequest {
    /// <p>the job ID of the job.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobResponse {
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobTemplateRequest {
    /// <p>The name of the job template.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetJobTemplateResponse {
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPresetRequest {
    /// <p>The name of the preset.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPresetResponse {
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetQueueRequest {
    /// <p>The name of the queue.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetQueueResponse {
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value H_264.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H264Settings {
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Average bitrate in bits/second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    #[serde(rename = "EntropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    #[serde(rename = "FieldEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_encoding: Option<String>,
    #[serde(rename = "FlickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use framerate conversion, specify the framerate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use framerate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    #[serde(rename = "GopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "HrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "HrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "MinIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "NumberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Number of reference frames to use. The encoder may use more than requested if using B-frames and/or interlaced encoding.</p>
    #[serde(rename = "NumberReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_reference_frames: Option<i64>,
    #[serde(rename = "ParControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "RepeatPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_pps: Option<String>,
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Softness. Selects quantizer matrix, larger values reduce high-frequency content in the encoded image.</p>
    #[serde(rename = "Softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    #[serde(rename = "UnregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
}

/// <p>Settings for H265 codec</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H265Settings {
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "AlternateTransferFunctionSei")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_transfer_function_sei: Option<String>,
    /// <p>Average bitrate in bits/second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    #[serde(rename = "FlickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Framerate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    #[serde(rename = "GopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "HrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "HrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "MinIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "NumberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Number of reference frames to use. The encoder may use more than requested if using B-frames and/or interlaced encoding.</p>
    #[serde(rename = "NumberReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_reference_frames: Option<i64>,
    #[serde(rename = "ParControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "SampleAdaptiveOffsetFilterMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_adaptive_offset_filter_mode: Option<String>,
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    #[serde(rename = "TemporalIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_ids: Option<String>,
    #[serde(rename = "Tiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<String>,
    #[serde(rename = "UnregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
    #[serde(rename = "WriteMp4PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_mp_4_packaging_type: Option<String>,
}

/// <p>Use the HDR master display (Hdr10Metadata) settings to correct HDR metadata or to provide missing metadata. These values vary depending on the input video and must be provided by a color grader. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that these settings are not color correction. Note that if you are creating HDR outputs inside of an HLS CMAF package, to comply with the Apple specification, you must use the HVC1 for H.265 setting.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hdr10Metadata {
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "BluePrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "BluePrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_primary_y: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "GreenPrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "GreenPrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_primary_y: Option<i64>,
    /// <p>Maximum light level among all samples in the coded video sequence, in units of candelas per square meter.</p>
    #[serde(rename = "MaxContentLightLevel")]
    pub max_content_light_level: i64,
    /// <p>Maximum average light level of any frame in the coded video sequence, in units of candelas per square meter.</p>
    #[serde(rename = "MaxFrameAverageLightLevel")]
    pub max_frame_average_light_level: i64,
    /// <p>Nominal maximum mastering display luminance in units of of 0.0001 candelas per square meter.</p>
    #[serde(rename = "MaxLuminance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_luminance: Option<i64>,
    /// <p>Nominal minimum mastering display luminance in units of of 0.0001 candelas per square meter</p>
    #[serde(rename = "MinLuminance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_luminance: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "RedPrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "RedPrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_primary_y: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "WhitePointX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_point_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "WhitePointY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_point_y: Option<i64>,
}

/// <p>Caption Language Mapping</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsCaptionLanguageMapping {
    /// <p>Caption channel.</p>
    #[serde(rename = "CaptionChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_channel: Option<i64>,
    /// <p>Specify the language for this caption channel, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Caption language description.</p>
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Settings for HLS encryption</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsEncryptionSettings {
    /// <p>This is a 128-bit, 16-byte hex value represented by a 32-character text string. If this parameter is not set then the Initialization Vector will follow the segment number by default.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    #[serde(rename = "InitializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
    #[serde(rename = "StaticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to HLS<em>GROUP</em>SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsGroupSettings {
    /// <p>Choose one or more ad marker types to pass SCTE35 signals through to this group of Apple HLS outputs.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    /// <p>A partial URI prefix that will be prepended to each output in the media .m3u8 file. Can be used if base manifest is delivered from a different URL than the main .m3u8 file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Language to be used on Caption outputs</p>
    #[serde(rename = "CaptionLanguageMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<HlsCaptionLanguageMapping>>,
    #[serde(rename = "CaptionLanguageSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<String>,
    #[serde(rename = "ClientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    #[serde(rename = "CodecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "DirectoryStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<String>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryptionSettings>,
    #[serde(rename = "ManifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    #[serde(rename = "ManifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>When set, Minimum Segment Size is enforced by looking ahead and back within the specified range for a nearby avail and extending the segment size if needed.</p>
    #[serde(rename = "MinSegmentLength")]
    pub min_segment_length: i64,
    #[serde(rename = "OutputSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<String>,
    #[serde(rename = "ProgramDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<String>,
    /// <p>Period of insertion of EXT-X-PROGRAM-DATE-TIME entry, in seconds.</p>
    #[serde(rename = "ProgramDateTimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i64>,
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of MPEG-2 Transport Stream segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer.</p>
    #[serde(rename = "SegmentLength")]
    pub segment_length: i64,
    /// <p>Number of segments to write to a subdirectory before starting a new one. directoryStructure must be SINGLE_DIRECTORY for this setting to have an effect.</p>
    #[serde(rename = "SegmentsPerSubdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_per_subdirectory: Option<i64>,
    #[serde(rename = "StreamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    #[serde(rename = "TimedMetadataId3Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_frame: Option<String>,
    /// <p>Timed Metadata interval in seconds.</p>
    #[serde(rename = "TimedMetadataId3Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_period: Option<i64>,
    /// <p>Provides an extra millisecond delta offset to fine tune the timestamps.</p>
    #[serde(rename = "TimestampDeltaMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_delta_milliseconds: Option<i64>,
}

/// <p>Settings for HLS output groups</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsSettings {
    /// <p>Specifies the group to which the audio Rendition belongs.</p>
    #[serde(rename = "AudioGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    /// <p>List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by &#39;,&#39;.</p>
    #[serde(rename = "AudioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    #[serde(rename = "AudioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    #[serde(rename = "IFrameOnlyManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_frame_only_manifest: Option<String>,
    /// <p>String concatenated to end of segment filenames. Accepts &quot;Format Identifiers&quot;:#format<em>identifier</em>parameters.</p>
    #[serde(rename = "SegmentModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_modifier: Option<String>,
}

/// <p>To insert ID3 tags in your output, specify two values. Use ID3 tag (Id3) to specify the base 64 encoded string and use Timecode (TimeCode) to specify the time when the tag should be inserted. To insert multiple ID3 tags in your output, create multiple instances of ID3 insertion (Id3Insertion).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id3Insertion {
    /// <p>Use ID3 tag (Id3) to provide a tag value in base64-encode format.</p>
    #[serde(rename = "Id3")]
    pub id_3: String,
    /// <p>Provide a Timecode (TimeCode) in HH:MM:SS:FF or HH:MM:SS;FF format.</p>
    #[serde(rename = "Timecode")]
    pub timecode: String,
}

/// <p>Enable the Image inserter (ImageInserter) feature to include a graphic overlay on your video. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInserter {
    /// <p>Image to insert. Must be 32 bit windows BMP, PNG, or TGA file. Must not be  larger than the output frames.</p>
    #[serde(rename = "InsertableImages")]
    pub insertable_images: Vec<InsertableImage>,
}

/// <p>Specifies media input</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    /// <p>Specifies set of audio selectors within an input to combine. An input may have multiple audio selector groups. See &quot;Audio Selector Group&quot;:#inputs-audio<em>selector</em>group for more information.</p>
    #[serde(rename = "AudioSelectorGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_groups: Option<::std::collections::HashMap<String, AudioSelectorGroup>>,
    /// <p>Use Audio selectors (AudioSelectors) to specify a track or set of tracks from the input that you will use in your outputs. You can use mutiple Audio selectors per input.</p>
    #[serde(rename = "AudioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<::std::collections::HashMap<String, AudioSelector>>,
    /// <p>Use Captions selectors (CaptionSelectors) to specify the captions data from the input that you will use in your outputs. You can use mutiple captions selectors per input.</p>
    #[serde(rename = "CaptionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<::std::collections::HashMap<String, CaptionSelector>>,
    #[serde(rename = "DeblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    #[serde(rename = "DenoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Use Input (fileInput) to define the source file used in the transcode job. There can be multiple inputs in a job. These inputs are concantenated, in the order they are specified in the job, to create the output.</p>
    #[serde(rename = "FileInput")]
    pub file_input: String,
    #[serde(rename = "FilterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "FilterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "InputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    #[serde(rename = "PsiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    #[serde(rename = "TimecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    #[serde(rename = "VideoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>To transcode only portions of your input (clips), include one Input clipping (one instance of InputClipping in the JSON job file) for each input clip. All input clips you specify will be included in every output of the job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputClipping {
    /// <p>Set End timecode (EndTimecode) to the end of the portion of the input you are clipping. The frame corresponding to the End timecode value is included in the clip. Start timecode or End timecode may be left blank, but not both. Use the format HH:MM:SS:FF or HH:MM:SS;FF, where HH is the hour, MM is the minute, SS is the second, and FF is the frame number. When choosing this value, take into account your setting for timecode source under input settings (InputTimecodeSource). For example, if you have embedded timecodes that start at 01:00:00:00 and you want your clip to end six minutes into the video, use 01:06:00:00.</p>
    #[serde(rename = "EndTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timecode: Option<String>,
    /// <p>Set Start timecode (StartTimecode) to the beginning of the portion of the input you are clipping. The frame corresponding to the Start timecode value is included in the clip. Start timecode or End timecode may be left blank, but not both. Use the format HH:MM:SS:FF or HH:MM:SS;FF, where HH is the hour, MM is the minute, SS is the second, and FF is the frame number. When choosing this value, take into account your setting for Input timecode source. For example, if you have embedded timecodes that start at 01:00:00:00 and you want your clip to begin five minutes into the video, use 01:05:00:00.</p>
    #[serde(rename = "StartTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timecode: Option<String>,
}

/// <p>Specified video input in a template.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputTemplate {
    /// <p>Specifies set of audio selectors within an input to combine. An input may have multiple audio selector groups. See &quot;Audio Selector Group&quot;:#inputs-audio<em>selector</em>group for more information.</p>
    #[serde(rename = "AudioSelectorGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_groups: Option<::std::collections::HashMap<String, AudioSelectorGroup>>,
    /// <p>Use Audio selectors (AudioSelectors) to specify a track or set of tracks from the input that you will use in your outputs. You can use mutiple Audio selectors per input.</p>
    #[serde(rename = "AudioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<::std::collections::HashMap<String, AudioSelector>>,
    /// <p>Use Captions selectors (CaptionSelectors) to specify the captions data from the input that you will use in your outputs. You can use mutiple captions selectors per input.</p>
    #[serde(rename = "CaptionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<::std::collections::HashMap<String, CaptionSelector>>,
    #[serde(rename = "DeblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    #[serde(rename = "DenoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    #[serde(rename = "FilterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "FilterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "InputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    #[serde(rename = "PsiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    #[serde(rename = "TimecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    #[serde(rename = "VideoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>Settings for Insertable Image</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertableImage {
    /// <p>Use Duration (Duration) to set the time, in milliseconds, for the image to remain on the output video.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Use Fade in (FadeIut) to set the length, in milliseconds, of the inserted image fade in. If you don&#39;t specify a value for Fade in, the image will appear abruptly at the Start time.</p>
    #[serde(rename = "FadeIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i64>,
    /// <p>Use Fade out (FadeOut) to set the length, in milliseconds, of the inserted image fade out. If you don&#39;t specify a value for Fade out, the image will disappear abruptly at the end of the inserted image duration.</p>
    #[serde(rename = "FadeOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i64>,
    /// <p>Specify the Height (Height) of the inserted image. Use a value that is less than or equal to the video resolution height. Leave this setting blank to use the native height of the image.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Use Image location (imageInserterInput) to specify the Amazon S3 location of the image to be inserted into the output. Use a 32 bit BMP, PNG, or TGA file that fits inside the video frame.</p>
    #[serde(rename = "ImageInserterInput")]
    pub image_inserter_input: String,
    /// <p>Use Left (ImageX) to set the distance, in pixels, between the inserted image and the left edge of the frame. Required for BMP, PNG and TGA input.</p>
    #[serde(rename = "ImageX")]
    pub image_x: i64,
    /// <p>Use Top (ImageY) to set the distance, in pixels, between the inserted image and the top edge of the video frame. Required for BMP, PNG and TGA input.</p>
    #[serde(rename = "ImageY")]
    pub image_y: i64,
    /// <p>Use Layer (Layer) to specify how overlapping inserted images appear. Images with higher values of layer appear on top of images with lower values of layer.</p>
    #[serde(rename = "Layer")]
    pub layer: i64,
    /// <p>Use Opacity (Opacity) to specify how much of the underlying video shows through the inserted image. 0 is transparent and 100 is fully opaque. Default is 50.</p>
    #[serde(rename = "Opacity")]
    pub opacity: i64,
    /// <p>Use Start time (StartTime) to specify the video timecode when the image is inserted in the output. This must be in timecode (HH:MM:SS:FF or HH:MM:SS;FF) format.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>Specify the Width (Width) of the inserted image. Use a value that is less than or equal to the video resolution width. Leave this setting blank to use the native width of the image.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Job {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time, in Unix epoch format in seconds, when the job got created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>Error code for the job</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    /// <p>Error message of Job</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>A portion of the job&#39;s ARN, unique within your AWS Elemental MediaConvert resources</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The job template that the job is created from, if it is created from a job template.</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>List of output group details</p>
    #[serde(rename = "OutputGroupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_details: Option<Vec<OutputGroupDetail>>,
    /// <p>Optional. When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html</p>
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Settings")]
    pub settings: JobSettings,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    /// <p>User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.</p>
    #[serde(rename = "UserMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>JobSettings contains all the transcode settings for a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobSettings {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.</p>
    #[serde(rename = "AdAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>Settings for ad avail blanking.  Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.</p>
    #[serde(rename = "AvailBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    /// <p>Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.</p>
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Input>,
    #[serde(rename = "NielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "OutputGroups")]
    pub output_groups: Vec<OutputGroup>,
    /// <p>Contains settings used to acquire and adjust timecode information from inputs.</p>
    #[serde(rename = "TimecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    #[serde(rename = "TimedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

/// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobTemplate {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional category you create to organize your job templates.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The timestamp in epoch seconds for Job template creation.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description you create for each job template.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds when the Job template was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name you create for each job template. Each name must be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: JobTemplateSettings,
    /// <p>A job template can be of two types: system or custom. System or built-in job templates can&#39;t be modified or deleted by the user.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobTemplateSettings {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.</p>
    #[serde(rename = "AdAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>Settings for ad avail blanking.  Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.</p>
    #[serde(rename = "AvailBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    /// <p>Use Inputs (inputs) to define the source file used in the transcode job. There can only be one input in a job template.  Using the API, you can include multiple inputs when referencing a job template.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputTemplate>>,
    #[serde(rename = "NielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "OutputGroups")]
    pub output_groups: Vec<OutputGroup>,
    /// <p>Contains settings used to acquire and adjust timecode information from inputs.</p>
    #[serde(rename = "TimecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    #[serde(rename = "TimedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobTemplatesRequest {
    /// <p>Optionally, specify a job template category to limit responses to only job templates from that category.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "ListBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of job templates, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of job templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListJobTemplatesResponse {
    /// <p>List of Job templates.</p>
    #[serde(rename = "JobTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_templates: Option<Vec<JobTemplate>>,
    /// <p>Use this string to request the next batch of job templates.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>Optional. Number of jobs, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>Provide a queue name to get back only jobs from that queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>List of jobs</p>
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>Use this string to request the next batch of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPresetsRequest {
    /// <p>Optionally, specify a preset category to limit responses to only presets from that category.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "ListBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of presets, up to twenty, that will be returned at one time</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of presets.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListPresetsResponse {
    /// <p>Use this string to request the next batch of presets.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of presets</p>
    #[serde(rename = "Presets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<Preset>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListQueuesRequest {
    #[serde(rename = "ListBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of queues, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of queues.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListQueuesResponse {
    /// <p>Use this string to request the next batch of queues.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of queues</p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<Queue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to list tags for. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<ResourceTags>,
}

/// <p>Settings for M2TS Container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M2tsSettings {
    #[serde(rename = "AudioBufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "AudioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary audio stream(s) in the transport stream. Multiple values are accepted, and can be entered in ranges and/or by comma separation.</p>
    #[serde(rename = "AudioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    /// <p>The output bitrate of the transport stream in bits per second. Setting to 0 lets the muxer automatically determine the appropriate bitrate. Other common values are 3750000, 7500000, and 15000000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "BufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<String>,
    #[serde(rename = "DvbNitSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    #[serde(rename = "DvbSdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    /// <p>Packet Identifier (PID) for input source DVB Subtitle data to this output. Multiple values are accepted, and can be entered in ranges and/or by comma separation.</p>
    #[serde(rename = "DvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<Vec<i64>>,
    #[serde(rename = "DvbTdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    /// <p>Packet Identifier (PID) for input source DVB Teletext data to this output.</p>
    #[serde(rename = "DvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<i64>,
    #[serde(rename = "EbpAudioInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<String>,
    #[serde(rename = "EbpPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<String>,
    #[serde(rename = "EsRateInPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    /// <p>The length in seconds of each fragment. Only used with EBP markers.</p>
    #[serde(rename = "FragmentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,
    /// <p>Maximum time in milliseconds between Program Clock References (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "MaxPcrInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pcr_interval: Option<i64>,
    /// <p>When set, enforces that Encoder Boundary Points do not come within the specified time interval of each other by looking ahead at input video. If another EBP is going to come in within the specified time interval, the current EBP is not emitted, and the segment is &quot;stretched&quot; to the next marker. The lookahead value does not add latency to the system. The Live Event must be configured elsewhere to create sufficient latency to make the lookahead accurate.</p>
    #[serde(rename = "MinEbpInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ebp_interval: Option<i64>,
    #[serde(rename = "NielsenId3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3: Option<String>,
    /// <p>Value in bits per second of extra null packets to insert into the transport stream. This can be used if a downstream encryption system requires periodic null packets.</p>
    #[serde(rename = "NullPacketBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_packet_bitrate: Option<f64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "PatInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    #[serde(rename = "PcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Packet Identifier (PID) of the Program Clock Reference (PCR) in the transport stream. When no value is given, the encoder will assign the same value as the Video PID.</p>
    #[serde(rename = "PcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "PmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Packet Identifier (PID) for the Program Map Table (PMT) in the transport stream.</p>
    #[serde(rename = "PmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    /// <p>Packet Identifier (PID) of the private metadata stream in the transport stream.</p>
    #[serde(rename = "PrivateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    /// <p>The value of the program number field in the Program Map Table.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    #[serde(rename = "RateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<String>,
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream.</p>
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    #[serde(rename = "SegmentationMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<String>,
    #[serde(rename = "SegmentationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<String>,
    /// <p>The length in seconds of each segment. Required unless markers is set to <em>none</em>.</p>
    #[serde(rename = "SegmentationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,
    /// <p>Packet Identifier (PID) of the timed metadata stream in the transport stream.</p>
    #[serde(rename = "TimedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    /// <p>The value of the transport stream ID field in the Program Map Table.</p>
    #[serde(rename = "TransportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary video stream in the transport stream.</p>
    #[serde(rename = "VideoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i64>,
}

/// <p>Settings for TS segments in HLS</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M3u8Settings {
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "AudioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary audio stream(s) in the transport stream. Multiple values are accepted, and can be entered in ranges and/or by comma separation.</p>
    #[serde(rename = "AudioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    #[serde(rename = "NielsenId3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "PatInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    #[serde(rename = "PcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Packet Identifier (PID) of the Program Clock Reference (PCR) in the transport stream. When no value is given, the encoder will assign the same value as the Video PID.</p>
    #[serde(rename = "PcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "PmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Packet Identifier (PID) for the Program Map Table (PMT) in the transport stream.</p>
    #[serde(rename = "PmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    /// <p>Packet Identifier (PID) of the private metadata stream in the transport stream.</p>
    #[serde(rename = "PrivateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    /// <p>The value of the program number field in the Program Map Table.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream.</p>
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    #[serde(rename = "TimedMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata: Option<String>,
    /// <p>Packet Identifier (PID) of the timed metadata stream in the transport stream.</p>
    #[serde(rename = "TimedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    /// <p>The value of the transport stream ID field in the Program Map Table.</p>
    #[serde(rename = "TransportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary video stream in the transport stream.</p>
    #[serde(rename = "VideoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i64>,
}

/// <p>Settings for MOV Container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovSettings {
    #[serde(rename = "ClapAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clap_atom: Option<String>,
    #[serde(rename = "CslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    #[serde(rename = "Mpeg2FourCCControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_four_cc_control: Option<String>,
    #[serde(rename = "PaddingControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_control: Option<String>,
    #[serde(rename = "Reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value MP2.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp2Settings {
    /// <p>Average bitrate in bits/second.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Set Channels to specify the number of channels in this output audio track. Choosing Mono in the console will give you 1 output channel; choosing Stereo will give you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Settings for MP4 Container</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp4Settings {
    #[serde(rename = "CslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    #[serde(rename = "FreeSpaceBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space_box: Option<String>,
    #[serde(rename = "MoovPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moov_placement: Option<String>,
    /// <p>Overrides the &quot;Major Brand&quot; field in the output file. Usually not necessary to specify.</p>
    #[serde(rename = "Mp4MajorBrand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_major_brand: Option<String>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value MPEG2.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mpeg2Settings {
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Average bitrate in bits/second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Framerate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "GopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "GopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "GopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "HrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "HrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    #[serde(rename = "IntraDcPrecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_dc_precision: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000.</p>
    #[serde(rename = "MaxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "MinIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "NumberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    #[serde(rename = "ParControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Softness. Selects quantizer matrix, larger values reduce high-frequency content in the encoded image.</p>
    #[serde(rename = "Softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
}

/// <p>If you are using DRM, set DRM System (MsSmoothEncryptionSettings) to specify the value SpekeKeyProvider.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsSmoothEncryptionSettings {
    #[serde(rename = "SpekeKeyProvider")]
    pub speke_key_provider: SpekeKeyProvider,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to MS<em>SMOOTH</em>GROUP_SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsSmoothGroupSettings {
    #[serde(rename = "AudioDeduplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_deduplication: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MsSmoothEncryptionSettings>,
    /// <p>Use Fragment length (FragmentLength) to specify the mp4 fragment sizes in seconds. Fragment length must be compatible with GOP size and framerate.</p>
    #[serde(rename = "FragmentLength")]
    pub fragment_length: i64,
    #[serde(rename = "ManifestEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_encoding: Option<String>,
}

/// <p>Settings for Nielsen Configuration</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NielsenConfiguration {
    /// <p>Use Nielsen Configuration (NielsenConfiguration) to set the Nielsen measurement system breakout code. Supported values are 0, 3, 7, and 9.</p>
    #[serde(rename = "BreakoutCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakout_code: Option<i64>,
    /// <p>Use Distributor ID (DistributorID) to specify the distributor ID that is assigned to your organization by Neilsen.</p>
    #[serde(rename = "DistributorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor_id: Option<String>,
}

/// <p>Enable the Noise reducer (NoiseReducer) feature to remove noise from your video output if necessary. Enable or disable this feature for each output individually. This setting is disabled by default. When you enable Noise reducer (NoiseReducer), you must also select a value for Noise reducer filter (NoiseReducerFilter).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseReducer {
    #[serde(rename = "Filter")]
    pub filter: String,
    #[serde(rename = "FilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<NoiseReducerFilterSettings>,
    #[serde(rename = "SpatialFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_filter_settings: Option<NoiseReducerSpatialFilterSettings>,
}

/// <p>Settings for a noise reducer filter</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseReducerFilterSettings {
    /// <p>Relative strength of noise reducing filter. Higher values produce stronger filtering.</p>
    #[serde(rename = "Strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i64>,
}

/// <p>Noise reducer filter settings for spatial filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseReducerSpatialFilterSettings {
    /// <p>Specify strength of post noise reduction sharpening filter, with 0 disabling the filter and 3 enabling it at maximum strength.</p>
    #[serde(rename = "PostFilterSharpenStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpen_strength: Option<i64>,
    /// <p>The speed of the filter, from -2 (lower speed) to 3 (higher speed), with 0 being the nominal value.</p>
    #[serde(rename = "Speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// <p>Relative strength of noise reducing filter. Higher values produce stronger filtering.</p>
    #[serde(rename = "Strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i64>,
}

/// <p>An output object describes the settings for a single output file or stream in an output group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    /// <p>(AudioDescriptions) contains groups of audio encoding settings organized by audio codec. Include one instance of (AudioDescriptions) per output. (AudioDescriptions) can contain multiple groups of encoding settings.</p>
    #[serde(rename = "AudioDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,
    /// <p>(CaptionDescriptions) contains groups of captions settings. For each output that has captions, include one instance of (CaptionDescriptions). (CaptionDescriptions) can contain multiple groups of captions settings.</p>
    #[serde(rename = "CaptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,
    #[serde(rename = "ContainerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    /// <p>Use Extension (Extension) to specify the file extension for outputs in File output groups. If you do not specify a value, the service will use default extensions by container type as follows * MPEG-2 transport stream, m2ts * Quicktime, mov * MXF container, mxf * MPEG-4 container, mp4 * No Container, the service will use codec extensions (e.g. AAC, H265, H265, AC3)</p>
    #[serde(rename = "Extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// <p>Use Name modifier (NameModifier) to have the service add a string to the end of each output filename. You specify the base filename as part of your destination URI. When you create multiple outputs in the same output group, Name modifier (NameModifier) is required. Name modifier also accepts format identifiers. For DASH ISO outputs, if you use the format identifiers $Number$ or $Time$ in one output, you must use them in the same way in all outputs of the output group.</p>
    #[serde(rename = "NameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
    #[serde(rename = "OutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_settings: Option<OutputSettings>,
    /// <p>Use Preset (Preset) to specifiy a preset for your transcoding settings. Provide the system or custom preset name. You can specify either Preset (Preset) or Container settings (ContainerSettings), but not both.</p>
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// <p>(VideoDescription) contains a group of video encoding settings. The specific video settings depend on the video codec you choose when you specify a value for Video codec (codec). Include one instance of (VideoDescription) per output.</p>
    #[serde(rename = "VideoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>OutputChannel mapping settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputChannelMapping {
    /// <p>List of input channels</p>
    #[serde(rename = "InputChannels")]
    pub input_channels: Vec<i64>,
}

/// <p>Details regarding output</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OutputDetail {
    /// <p>Duration in milliseconds</p>
    #[serde(rename = "DurationInMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    #[serde(rename = "VideoDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_details: Option<VideoDetail>,
}

/// <p>Group of outputs</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputGroup {
    /// <p>Use Custom Group Name (CustomName) to specify a name for the output group. This value is displayed on the console and can make your job settings JSON more human-readable. It does not affect your outputs. Use up to twelve characters that are either letters, numbers, spaces, or underscores.</p>
    #[serde(rename = "CustomName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// <p>Name of the output group</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputGroupSettings")]
    pub output_group_settings: OutputGroupSettings,
    /// <p>This object holds groups of encoding settings, one group of settings per output.</p>
    #[serde(rename = "Outputs")]
    pub outputs: Vec<Output>,
}

/// <p>Contains details about the output groups specified in the job settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OutputGroupDetail {
    /// <p>Details about the output</p>
    #[serde(rename = "OutputDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<Vec<OutputDetail>>,
}

/// <p>Output Group settings, including type</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputGroupSettings {
    #[serde(rename = "CmafGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_group_settings: Option<CmafGroupSettings>,
    #[serde(rename = "DashIsoGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_iso_group_settings: Option<DashIsoGroupSettings>,
    #[serde(rename = "FileGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_group_settings: Option<FileGroupSettings>,
    #[serde(rename = "HlsGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    #[serde(rename = "MsSmoothGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Specific settings for this type of output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputSettings {
    #[serde(rename = "HlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_settings: Option<HlsSettings>,
}

/// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Preset {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional category you create to organize your presets.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The timestamp in epoch seconds for preset creation.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description you create for each preset.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds when the preset was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name you create for each preset. Each name must be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settings")]
    pub settings: PresetSettings,
    /// <p>A preset can be of two types: system or custom. System or built-in preset can&#39;t be modified or deleted by the user.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Settings for preset</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresetSettings {
    /// <p>(AudioDescriptions) contains groups of audio encoding settings organized by audio codec. Include one instance of (AudioDescriptions) per output. (AudioDescriptions) can contain multiple groups of encoding settings.</p>
    #[serde(rename = "AudioDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,
    /// <p>Caption settings for this preset. There can be multiple caption settings in a single output.</p>
    #[serde(rename = "CaptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescriptionPreset>>,
    #[serde(rename = "ContainerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    /// <p>(VideoDescription) contains a group of video encoding settings. The specific video settings depend on the video codec you choose when you specify a value for Video codec (codec). Include one instance of (VideoDescription) per output.</p>
    #[serde(rename = "VideoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value PRORES.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProresSettings {
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Framerate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use framerate conversion, specify the framerate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    #[serde(rename = "ParControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "ParDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "ParNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
}

/// <p>MediaConvert jobs are submitted to a queue. Unless specified otherwise jobs are submitted to a built-in default queue. User can create additional queues to separate the jobs of different categories or priority.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Queue {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp in epoch seconds for queue creation.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description you create for each queue.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds when the queue was last updated.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name you create for each queue. Each name must be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Estimated number of jobs in PROGRESSING status.</p>
    #[serde(rename = "ProgressingJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progressing_jobs_count: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Estimated number of jobs in SUBMITTED status.</p>
    #[serde(rename = "SubmittedJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_jobs_count: Option<i64>,
    /// <p>A queue can be of two types: system or custom. System or built-in queues can&#39;t be modified or deleted by the user.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Use Rectangle to identify a specific area of the video frame.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    /// <p>Height of rectangle in pixels.</p>
    #[serde(rename = "Height")]
    pub height: i64,
    /// <p>Width of rectangle in pixels.</p>
    #[serde(rename = "Width")]
    pub width: i64,
    /// <p>The distance, in pixels, between the rectangle and the left edge of the video frame.</p>
    #[serde(rename = "X")]
    pub x: i64,
    /// <p>The distance, in pixels, between the rectangle and the top edge of the video frame.</p>
    #[serde(rename = "Y")]
    pub y: i64,
}

/// <p>Use Manual audio remixing (RemixSettings) to adjust audio levels for each audio channel in each output of your job. With audio remixing, you can output more or fewer audio channels than your input audio source provides.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemixSettings {
    #[serde(rename = "ChannelMapping")]
    pub channel_mapping: ChannelMapping,
    /// <p>Specify the number of audio channels from your input that you want to use in your output. With remixing, you might combine or split the data in these channels, so the number of channels in your final output might be different.</p>
    #[serde(rename = "ChannelsIn")]
    pub channels_in: i64,
    /// <p>Specify the number of channels in this output after remixing. Valid values: 1, 2, 4, 6, 8</p>
    #[serde(rename = "ChannelsOut")]
    pub channels_out: i64,
}

/// <p>The Amazon Resource Name (ARN) and tags for an AWS Elemental MediaConvert resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceTags {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Settings for SCC caption output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SccDestinationSettings {
    #[serde(rename = "Framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<String>,
}

/// <p>Settings for use with a SPEKE key provider</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProvider {
    /// <p>The SPEKE-compliant server uses Resource ID (ResourceId) to identify content.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Relates to SPEKE implementation. DRM system identifiers. DASH output groups support a max of two system ids. Other group types support one system id.</p>
    #[serde(rename = "SystemIds")]
    pub system_ids: Vec<String>,
    /// <p>Use URL (Url) to specify the SPEKE-compliant server that will provide keys for content.</p>
    #[serde(rename = "Url")]
    pub url: String,
}

/// <p>Settings for use with a SPEKE key provider.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticKeyProvider {
    /// <p>Relates to DRM implementation. Sets the value of the KEYFORMAT attribute. Must be &#39;identity&#39; or a reverse DNS string. May be omitted to indicate an implicit value of &#39;identity&#39;.</p>
    #[serde(rename = "KeyFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    /// <p>Relates to DRM implementation. Either a single positive integer version value or a slash delimited list of version values (1/2/3).</p>
    #[serde(rename = "KeyFormatVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format_versions: Option<String>,
    /// <p>Relates to DRM implementation. Use a 32-character hexidecimal string to specify Key Value (StaticKeyValue).</p>
    #[serde(rename = "StaticKeyValue")]
    pub static_key_value: String,
    /// <p>Relates to DRM implementation. The location of the license server used for protecting content.</p>
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Settings for Teletext caption output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeletextDestinationSettings {
    /// <p>Set pageNumber to the Teletext page number for the destination captions for this output. This value must be a three-digit hexadecimal string; strings ending in -FF are invalid. If you are passing through the entire set of Teletext data, do not use this field.</p>
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

/// <p>Settings specific to Teletext caption sources, including Page number.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeletextSourceSettings {
    /// <p>Use Page Number (PageNumber) to specify the three-digit hexadecimal page number that will be used for Teletext captions. Do not use this setting if you are passing through teletext from the input source to output.</p>
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

/// <p>Timecode burn-in (TimecodeBurnIn)--Burns the output timecode and specified prefix into the output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimecodeBurnin {
    /// <p>Use Font Size (FontSize) to set the font size of any burned-in timecode. Valid values are 10, 16, 32, 48.</p>
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>Use Prefix (Prefix) to place ASCII characters before any burned-in timecode. For example, a prefix of &quot;EZ-&quot; will result in the timecode &quot;EZ-00:00:00:00&quot;. Provide either the characters themselves or the ASCII code equivalents. The supported range of characters is 0x20 through 0x7e. This includes letters, numbers, and all special characters represented on a standard English keyboard.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>These settings control how the service handles timecodes throughout the job. These settings don&#39;t affect input clipping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimecodeConfig {
    /// <p>If you use an editing platform that relies on an anchor timecode, use Anchor Timecode (Anchor) to specify a timecode that will match the input video frame to the output video frame. Use 24-hour format with frame number, (HH:MM:SS:FF) or (HH:MM:SS;FF). This setting ignores framerate conversion. System behavior for Anchor Timecode varies depending on your setting for Source (TimecodeSource). * If Source (TimecodeSource) is set to Specified Start (SPECIFIEDSTART), the first input frame is the specified value in Start Timecode (Start). Anchor Timecode (Anchor) and Start Timecode (Start) are used calculate output timecode. * If Source (TimecodeSource) is set to Start at 0 (ZEROBASED)  the  first frame is 00:00:00:00. * If Source (TimecodeSource) is set to Embedded (EMBEDDED), the  first frame is the timecode value on the first input frame of the input.</p>
    #[serde(rename = "Anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>Only use when you set Source (TimecodeSource) to Specified start (SPECIFIEDSTART). Use Start timecode (Start) to specify the timecode for the initial frame. Use 24-hour format with frame number, (HH:MM:SS:FF) or (HH:MM:SS;FF).</p>
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>Only applies to outputs that support program-date-time stamp. Use Timestamp offset (TimestampOffset) to overwrite the timecode date without affecting the time and frame number. Provide the new date as a string in the format &quot;yyyy-mm-dd&quot;.  To use Time stamp offset, you must also enable Insert program-date-time (InsertProgramDateTime) in the output settings. For example, if the date part of your timecodes is 2002-1-25 and you want to change it to one year later, set Timestamp offset (TimestampOffset) to 2003-1-25.</p>
    #[serde(rename = "TimestampOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset: Option<String>,
}

/// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in your job. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimedMetadataInsertion {
    /// <p>Id3Insertions contains the array of Id3Insertion instances.</p>
    #[serde(rename = "Id3Insertions")]
    pub id_3_insertions: Vec<Id3Insertion>,
}

/// <p>Information about when jobs are submitted, started, and finished is specified in Unix epoch format in seconds.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Timing {
    /// <p>The time, in Unix epoch format, that the transcoding job finished</p>
    #[serde(rename = "FinishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,
    /// <p>The time, in Unix epoch format, that transcoding for the job began.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The time, in Unix epoch format, that you submitted the job.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Settings specific to TTML caption outputs, including Pass style information (TtmlStylePassthrough).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtmlDestinationSettings {
    #[serde(rename = "StylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to remove tags from. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The keys of the tags that you want to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobTemplateRequest {
    /// <p>The new category for the job template, if you are changing it.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The new description for the job template, if you are changing it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the job template you are modifying</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The new queue for the job template, if you are changing it.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<JobTemplateSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateJobTemplateResponse {
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePresetRequest {
    /// <p>The new category for the preset, if you are changing it.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The new description for the preset, if you are changing it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the preset you are modifying.</p>
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PresetSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdatePresetResponse {
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateQueueRequest {
    /// <p>The new description for the queue, if you are changing it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue you are modifying.</p>
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateQueueResponse {
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Video codec settings, (CodecSettings) under (VideoDescription), contains the group of settings related to video encoding. The settings in this group vary depending on the value you choose for Video codec (Codec). For each codec enum you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * H<em>264, H264Settings * H</em>265, H265Settings * MPEG2, Mpeg2Settings * PRORES, ProresSettings * FRAME_CAPTURE, FrameCaptureSettings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoCodecSettings {
    #[serde(rename = "Codec")]
    pub codec: String,
    #[serde(rename = "FrameCaptureSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    #[serde(rename = "H264Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,
    #[serde(rename = "H265Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,
    #[serde(rename = "Mpeg2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_settings: Option<Mpeg2Settings>,
    #[serde(rename = "ProresSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prores_settings: Option<ProresSettings>,
}

/// <p>Settings for video outputs</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoDescription {
    #[serde(rename = "AfdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "AntiAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_alias: Option<String>,
    #[serde(rename = "CodecSettings")]
    pub codec_settings: VideoCodecSettings,
    #[serde(rename = "ColorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Applies only if your input aspect ratio is different from your output aspect ratio. Use Input cropping rectangle (Crop) to specify the  video area the service will include in the output. This will crop the input source, causing video pixels to be removed on encode. Do not use this setting if you have enabled Stretch to output (stretchToOutput) in your output settings.</p>
    #[serde(rename = "Crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    #[serde(rename = "DropFrameTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_frame_timecode: Option<String>,
    /// <p>Applies only if you set AFD Signaling(AfdSignaling) to Fixed (FIXED). Use Fixed (FixedAfd) to specify a four-bit AFD value which the service will write on all  frames of this video output.</p>
    #[serde(rename = "FixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<i64>,
    /// <p>Use the Height (Height) setting to define the video resolution height for this output. Specify in pixels. If you don&#39;t provide a value here, the service will use the input height.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Use Position (Position) to point to a rectangle object to define your position. This setting overrides any other aspect ratio.</p>
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    #[serde(rename = "RespondToAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<String>,
    #[serde(rename = "ScalingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<String>,
    /// <p>Use Sharpness (Sharpness)setting to specify the strength of anti-aliasing. This setting changes the width of the anti-alias filter kernel used for scaling. Sharpness only applies if your output resolution is different from your input resolution, and if you set Anti-alias (AntiAlias) to ENABLED. 0 is the softest setting, 100 the sharpest, and 50 recommended for most content.</p>
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i64>,
    #[serde(rename = "TimecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
    /// <p>Find additional transcoding features under Preprocessors (VideoPreprocessors). Enable the features at each output individually. These features are disabled by default.</p>
    #[serde(rename = "VideoPreprocessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_preprocessors: Option<VideoPreprocessor>,
    /// <p>Use Width (Width) to define the video resolution width, in pixels, for this output. If you don&#39;t provide a value here, the service will use the input width.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Contains details about the output&#39;s video stream</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VideoDetail {
    /// <p>Height in pixels for the output</p>
    #[serde(rename = "HeightInPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_in_px: Option<i64>,
    /// <p>Width in pixels for the output</p>
    #[serde(rename = "WidthInPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_in_px: Option<i64>,
}

/// <p>Find additional transcoding features under Preprocessors (VideoPreprocessors). Enable the features at each output individually. These features are disabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoPreprocessor {
    /// <p>Enable the Color corrector (ColorCorrector) feature if necessary. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
    #[serde(rename = "ColorCorrector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_corrector: Option<ColorCorrector>,
    /// <p>Use Deinterlacer (Deinterlacer) to produce smoother motion and a clearer picture.</p>
    #[serde(rename = "Deinterlacer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deinterlacer: Option<Deinterlacer>,
    /// <p>Enable the Image inserter (ImageInserter) feature to include a graphic overlay on your video. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
    #[serde(rename = "ImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>Enable the Noise reducer (NoiseReducer) feature to remove noise from your video output if necessary. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
    #[serde(rename = "NoiseReducer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_reducer: Option<NoiseReducer>,
    /// <p>Timecode burn-in (TimecodeBurnIn)--Burns the output timecode and specified prefix into the output.</p>
    #[serde(rename = "TimecodeBurnin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin: Option<TimecodeBurnin>,
}

/// <p>Selector for video.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoSelector {
    #[serde(rename = "ColorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(rename = "ColorSpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<String>,
    #[serde(rename = "Hdr10Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_metadata: Option<Hdr10Metadata>,
    /// <p>Use PID (Pid) to select specific video data from an input file. Specify this value as an integer; the system automatically converts it to the hexidecimal value. For example, 257 selects PID 0x101. A PID, or packet identifier, is an identifier for a set of data in an MPEG-2 transport stream container.</p>
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// <p>Selects a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value WAV.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WavSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "BitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Set Channels to specify the number of channels in this output audio track. With WAV, valid values 1, 2, 4, and 8. In the console, these values are Mono, Stereo, 4-Channel, and 8-Channel, respectively.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sample rate in Hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl CancelJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CancelJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CancelJobError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return CancelJobError::Conflict(String::from(error_message)),
                "ForbiddenException" => {
                    return CancelJobError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CancelJobError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return CancelJobError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return CancelJobError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CancelJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CancelJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CancelJobError {
    fn from(err: serde_json::error::Error) -> CancelJobError {
        CancelJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelJobError {
    fn from(err: io::Error) -> CancelJobError {
        CancelJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::BadRequest(ref cause) => cause,
            CancelJobError::Conflict(ref cause) => cause,
            CancelJobError::Forbidden(ref cause) => cause,
            CancelJobError::InternalServerError(ref cause) => cause,
            CancelJobError::NotFound(ref cause) => cause,
            CancelJobError::TooManyRequests(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::ParseError(ref cause) => cause,
            CancelJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl CreateJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateJobError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return CreateJobError::Conflict(String::from(error_message)),
                "ForbiddenException" => {
                    return CreateJobError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateJobError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return CreateJobError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return CreateJobError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateJobError {
    fn from(err: serde_json::error::Error) -> CreateJobError {
        CreateJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobError {
    fn from(err: CredentialsError) -> CreateJobError {
        CreateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobError {
    fn from(err: HttpDispatchError) -> CreateJobError {
        CreateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobError {
    fn from(err: io::Error) -> CreateJobError {
        CreateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::BadRequest(ref cause) => cause,
            CreateJobError::Conflict(ref cause) => cause,
            CreateJobError::Forbidden(ref cause) => cause,
            CreateJobError::InternalServerError(ref cause) => cause,
            CreateJobError::NotFound(ref cause) => cause,
            CreateJobError::TooManyRequests(ref cause) => cause,
            CreateJobError::Validation(ref cause) => cause,
            CreateJobError::Credentials(ref err) => err.description(),
            CreateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobError::ParseError(ref cause) => cause,
            CreateJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateJobTemplate
#[derive(Debug, PartialEq)]
pub enum CreateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl CreateJobTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateJobTemplateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateJobTemplateError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return CreateJobTemplateError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return CreateJobTemplateError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateJobTemplateError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreateJobTemplateError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateJobTemplateError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateJobTemplateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateJobTemplateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateJobTemplateError {
    fn from(err: serde_json::error::Error) -> CreateJobTemplateError {
        CreateJobTemplateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobTemplateError {
    fn from(err: CredentialsError) -> CreateJobTemplateError {
        CreateJobTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobTemplateError {
    fn from(err: HttpDispatchError) -> CreateJobTemplateError {
        CreateJobTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobTemplateError {
    fn from(err: io::Error) -> CreateJobTemplateError {
        CreateJobTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateJobTemplateError::BadRequest(ref cause) => cause,
            CreateJobTemplateError::Conflict(ref cause) => cause,
            CreateJobTemplateError::Forbidden(ref cause) => cause,
            CreateJobTemplateError::InternalServerError(ref cause) => cause,
            CreateJobTemplateError::NotFound(ref cause) => cause,
            CreateJobTemplateError::TooManyRequests(ref cause) => cause,
            CreateJobTemplateError::Validation(ref cause) => cause,
            CreateJobTemplateError::Credentials(ref err) => err.description(),
            CreateJobTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateJobTemplateError::ParseError(ref cause) => cause,
            CreateJobTemplateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePreset
#[derive(Debug, PartialEq)]
pub enum CreatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl CreatePresetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreatePresetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreatePresetError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return CreatePresetError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return CreatePresetError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreatePresetError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreatePresetError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreatePresetError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreatePresetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreatePresetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePresetError {
    fn from(err: serde_json::error::Error) -> CreatePresetError {
        CreatePresetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePresetError {
    fn from(err: CredentialsError) -> CreatePresetError {
        CreatePresetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePresetError {
    fn from(err: HttpDispatchError) -> CreatePresetError {
        CreatePresetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePresetError {
    fn from(err: io::Error) -> CreatePresetError {
        CreatePresetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePresetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePresetError {
    fn description(&self) -> &str {
        match *self {
            CreatePresetError::BadRequest(ref cause) => cause,
            CreatePresetError::Conflict(ref cause) => cause,
            CreatePresetError::Forbidden(ref cause) => cause,
            CreatePresetError::InternalServerError(ref cause) => cause,
            CreatePresetError::NotFound(ref cause) => cause,
            CreatePresetError::TooManyRequests(ref cause) => cause,
            CreatePresetError::Validation(ref cause) => cause,
            CreatePresetError::Credentials(ref err) => err.description(),
            CreatePresetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePresetError::ParseError(ref cause) => cause,
            CreatePresetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateQueue
#[derive(Debug, PartialEq)]
pub enum CreateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl CreateQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return CreateQueueError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return CreateQueueError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return CreateQueueError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return CreateQueueError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return CreateQueueError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateQueueError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateQueueError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateQueueError {
    fn from(err: serde_json::error::Error) -> CreateQueueError {
        CreateQueueError::ParseError(err.description().to_string())
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
            CreateQueueError::BadRequest(ref cause) => cause,
            CreateQueueError::Conflict(ref cause) => cause,
            CreateQueueError::Forbidden(ref cause) => cause,
            CreateQueueError::InternalServerError(ref cause) => cause,
            CreateQueueError::NotFound(ref cause) => cause,
            CreateQueueError::TooManyRequests(ref cause) => cause,
            CreateQueueError::Validation(ref cause) => cause,
            CreateQueueError::Credentials(ref err) => err.description(),
            CreateQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateQueueError::ParseError(ref cause) => cause,
            CreateQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteJobTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl DeleteJobTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteJobTemplateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteJobTemplateError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return DeleteJobTemplateError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return DeleteJobTemplateError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteJobTemplateError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteJobTemplateError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteJobTemplateError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteJobTemplateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteJobTemplateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteJobTemplateError {
    fn from(err: serde_json::error::Error) -> DeleteJobTemplateError {
        DeleteJobTemplateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteJobTemplateError {
    fn from(err: CredentialsError) -> DeleteJobTemplateError {
        DeleteJobTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteJobTemplateError {
    fn from(err: HttpDispatchError) -> DeleteJobTemplateError {
        DeleteJobTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteJobTemplateError {
    fn from(err: io::Error) -> DeleteJobTemplateError {
        DeleteJobTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteJobTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteJobTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteJobTemplateError::BadRequest(ref cause) => cause,
            DeleteJobTemplateError::Conflict(ref cause) => cause,
            DeleteJobTemplateError::Forbidden(ref cause) => cause,
            DeleteJobTemplateError::InternalServerError(ref cause) => cause,
            DeleteJobTemplateError::NotFound(ref cause) => cause,
            DeleteJobTemplateError::TooManyRequests(ref cause) => cause,
            DeleteJobTemplateError::Validation(ref cause) => cause,
            DeleteJobTemplateError::Credentials(ref err) => err.description(),
            DeleteJobTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteJobTemplateError::ParseError(ref cause) => cause,
            DeleteJobTemplateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeletePreset
#[derive(Debug, PartialEq)]
pub enum DeletePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl DeletePresetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeletePresetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeletePresetError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return DeletePresetError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return DeletePresetError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeletePresetError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeletePresetError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeletePresetError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeletePresetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeletePresetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeletePresetError {
    fn from(err: serde_json::error::Error) -> DeletePresetError {
        DeletePresetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePresetError {
    fn from(err: CredentialsError) -> DeletePresetError {
        DeletePresetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePresetError {
    fn from(err: HttpDispatchError) -> DeletePresetError {
        DeletePresetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePresetError {
    fn from(err: io::Error) -> DeletePresetError {
        DeletePresetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePresetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePresetError {
    fn description(&self) -> &str {
        match *self {
            DeletePresetError::BadRequest(ref cause) => cause,
            DeletePresetError::Conflict(ref cause) => cause,
            DeletePresetError::Forbidden(ref cause) => cause,
            DeletePresetError::InternalServerError(ref cause) => cause,
            DeletePresetError::NotFound(ref cause) => cause,
            DeletePresetError::TooManyRequests(ref cause) => cause,
            DeletePresetError::Validation(ref cause) => cause,
            DeletePresetError::Credentials(ref err) => err.description(),
            DeletePresetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePresetError::ParseError(ref cause) => cause,
            DeletePresetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteQueue
#[derive(Debug, PartialEq)]
pub enum DeleteQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl DeleteQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DeleteQueueError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return DeleteQueueError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return DeleteQueueError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DeleteQueueError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return DeleteQueueError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteQueueError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteQueueError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteQueueError {
    fn from(err: serde_json::error::Error) -> DeleteQueueError {
        DeleteQueueError::ParseError(err.description().to_string())
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
            DeleteQueueError::BadRequest(ref cause) => cause,
            DeleteQueueError::Conflict(ref cause) => cause,
            DeleteQueueError::Forbidden(ref cause) => cause,
            DeleteQueueError::InternalServerError(ref cause) => cause,
            DeleteQueueError::NotFound(ref cause) => cause,
            DeleteQueueError::TooManyRequests(ref cause) => cause,
            DeleteQueueError::Validation(ref cause) => cause,
            DeleteQueueError::Credentials(ref err) => err.description(),
            DeleteQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteQueueError::ParseError(ref cause) => cause,
            DeleteQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl DescribeEndpointsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEndpointsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return DescribeEndpointsError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return DescribeEndpointsError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return DescribeEndpointsError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return DescribeEndpointsError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return DescribeEndpointsError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DescribeEndpointsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeEndpointsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeEndpointsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEndpointsError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointsError {
        DescribeEndpointsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointsError {
    fn from(err: CredentialsError) -> DescribeEndpointsError {
        DescribeEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointsError {
    fn from(err: HttpDispatchError) -> DescribeEndpointsError {
        DescribeEndpointsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEndpointsError {
    fn from(err: io::Error) -> DescribeEndpointsError {
        DescribeEndpointsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointsError::BadRequest(ref cause) => cause,
            DescribeEndpointsError::Conflict(ref cause) => cause,
            DescribeEndpointsError::Forbidden(ref cause) => cause,
            DescribeEndpointsError::InternalServerError(ref cause) => cause,
            DescribeEndpointsError::NotFound(ref cause) => cause,
            DescribeEndpointsError::TooManyRequests(ref cause) => cause,
            DescribeEndpointsError::Validation(ref cause) => cause,
            DescribeEndpointsError::Credentials(ref err) => err.description(),
            DescribeEndpointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEndpointsError::ParseError(ref cause) => cause,
            DescribeEndpointsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl GetJobError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetJobError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return GetJobError::Conflict(String::from(error_message)),
                "ForbiddenException" => return GetJobError::Forbidden(String::from(error_message)),
                "InternalServerErrorException" => {
                    return GetJobError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return GetJobError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetJobError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => return GetJobError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetJobError {
    fn from(err: serde_json::error::Error) -> GetJobError {
        GetJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobError {
    fn from(err: CredentialsError) -> GetJobError {
        GetJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobError {
    fn from(err: HttpDispatchError) -> GetJobError {
        GetJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobError {
    fn from(err: io::Error) -> GetJobError {
        GetJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::BadRequest(ref cause) => cause,
            GetJobError::Conflict(ref cause) => cause,
            GetJobError::Forbidden(ref cause) => cause,
            GetJobError::InternalServerError(ref cause) => cause,
            GetJobError::NotFound(ref cause) => cause,
            GetJobError::TooManyRequests(ref cause) => cause,
            GetJobError::Validation(ref cause) => cause,
            GetJobError::Credentials(ref err) => err.description(),
            GetJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobError::ParseError(ref cause) => cause,
            GetJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetJobTemplate
#[derive(Debug, PartialEq)]
pub enum GetJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl GetJobTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetJobTemplateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetJobTemplateError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return GetJobTemplateError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return GetJobTemplateError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetJobTemplateError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return GetJobTemplateError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetJobTemplateError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetJobTemplateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetJobTemplateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetJobTemplateError {
    fn from(err: serde_json::error::Error) -> GetJobTemplateError {
        GetJobTemplateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobTemplateError {
    fn from(err: CredentialsError) -> GetJobTemplateError {
        GetJobTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobTemplateError {
    fn from(err: HttpDispatchError) -> GetJobTemplateError {
        GetJobTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobTemplateError {
    fn from(err: io::Error) -> GetJobTemplateError {
        GetJobTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetJobTemplateError::BadRequest(ref cause) => cause,
            GetJobTemplateError::Conflict(ref cause) => cause,
            GetJobTemplateError::Forbidden(ref cause) => cause,
            GetJobTemplateError::InternalServerError(ref cause) => cause,
            GetJobTemplateError::NotFound(ref cause) => cause,
            GetJobTemplateError::TooManyRequests(ref cause) => cause,
            GetJobTemplateError::Validation(ref cause) => cause,
            GetJobTemplateError::Credentials(ref err) => err.description(),
            GetJobTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobTemplateError::ParseError(ref cause) => cause,
            GetJobTemplateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetPreset
#[derive(Debug, PartialEq)]
pub enum GetPresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl GetPresetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetPresetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetPresetError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return GetPresetError::Conflict(String::from(error_message)),
                "ForbiddenException" => {
                    return GetPresetError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetPresetError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return GetPresetError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetPresetError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetPresetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetPresetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPresetError {
    fn from(err: serde_json::error::Error) -> GetPresetError {
        GetPresetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPresetError {
    fn from(err: CredentialsError) -> GetPresetError {
        GetPresetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPresetError {
    fn from(err: HttpDispatchError) -> GetPresetError {
        GetPresetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPresetError {
    fn from(err: io::Error) -> GetPresetError {
        GetPresetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPresetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPresetError {
    fn description(&self) -> &str {
        match *self {
            GetPresetError::BadRequest(ref cause) => cause,
            GetPresetError::Conflict(ref cause) => cause,
            GetPresetError::Forbidden(ref cause) => cause,
            GetPresetError::InternalServerError(ref cause) => cause,
            GetPresetError::NotFound(ref cause) => cause,
            GetPresetError::TooManyRequests(ref cause) => cause,
            GetPresetError::Validation(ref cause) => cause,
            GetPresetError::Credentials(ref err) => err.description(),
            GetPresetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPresetError::ParseError(ref cause) => cause,
            GetPresetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetQueue
#[derive(Debug, PartialEq)]
pub enum GetQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl GetQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return GetQueueError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return GetQueueError::Conflict(String::from(error_message)),
                "ForbiddenException" => {
                    return GetQueueError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return GetQueueError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return GetQueueError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetQueueError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetQueueError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetQueueError {
    fn from(err: serde_json::error::Error) -> GetQueueError {
        GetQueueError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetQueueError {
    fn from(err: CredentialsError) -> GetQueueError {
        GetQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetQueueError {
    fn from(err: HttpDispatchError) -> GetQueueError {
        GetQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetQueueError {
    fn from(err: io::Error) -> GetQueueError {
        GetQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetQueueError {
    fn description(&self) -> &str {
        match *self {
            GetQueueError::BadRequest(ref cause) => cause,
            GetQueueError::Conflict(ref cause) => cause,
            GetQueueError::Forbidden(ref cause) => cause,
            GetQueueError::InternalServerError(ref cause) => cause,
            GetQueueError::NotFound(ref cause) => cause,
            GetQueueError::TooManyRequests(ref cause) => cause,
            GetQueueError::Validation(ref cause) => cause,
            GetQueueError::Credentials(ref err) => err.description(),
            GetQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetQueueError::ParseError(ref cause) => cause,
            GetQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListJobTemplates
#[derive(Debug, PartialEq)]
pub enum ListJobTemplatesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl ListJobTemplatesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListJobTemplatesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListJobTemplatesError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return ListJobTemplatesError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListJobTemplatesError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListJobTemplatesError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return ListJobTemplatesError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListJobTemplatesError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListJobTemplatesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListJobTemplatesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListJobTemplatesError {
    fn from(err: serde_json::error::Error) -> ListJobTemplatesError {
        ListJobTemplatesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobTemplatesError {
    fn from(err: CredentialsError) -> ListJobTemplatesError {
        ListJobTemplatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobTemplatesError {
    fn from(err: HttpDispatchError) -> ListJobTemplatesError {
        ListJobTemplatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobTemplatesError {
    fn from(err: io::Error) -> ListJobTemplatesError {
        ListJobTemplatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobTemplatesError {
    fn description(&self) -> &str {
        match *self {
            ListJobTemplatesError::BadRequest(ref cause) => cause,
            ListJobTemplatesError::Conflict(ref cause) => cause,
            ListJobTemplatesError::Forbidden(ref cause) => cause,
            ListJobTemplatesError::InternalServerError(ref cause) => cause,
            ListJobTemplatesError::NotFound(ref cause) => cause,
            ListJobTemplatesError::TooManyRequests(ref cause) => cause,
            ListJobTemplatesError::Validation(ref cause) => cause,
            ListJobTemplatesError::Credentials(ref err) => err.description(),
            ListJobTemplatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobTemplatesError::ParseError(ref cause) => cause,
            ListJobTemplatesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl ListJobsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListJobsError::BadRequest(String::from(error_message));
                }
                "ConflictException" => return ListJobsError::Conflict(String::from(error_message)),
                "ForbiddenException" => {
                    return ListJobsError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListJobsError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => return ListJobsError::NotFound(String::from(error_message)),
                "TooManyRequestsException" => {
                    return ListJobsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListJobsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::BadRequest(ref cause) => cause,
            ListJobsError::Conflict(ref cause) => cause,
            ListJobsError::Forbidden(ref cause) => cause,
            ListJobsError::InternalServerError(ref cause) => cause,
            ListJobsError::NotFound(ref cause) => cause,
            ListJobsError::TooManyRequests(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::ParseError(ref cause) => cause,
            ListJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListPresets
#[derive(Debug, PartialEq)]
pub enum ListPresetsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl ListPresetsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListPresetsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListPresetsError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return ListPresetsError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListPresetsError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListPresetsError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return ListPresetsError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListPresetsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListPresetsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListPresetsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPresetsError {
    fn from(err: serde_json::error::Error) -> ListPresetsError {
        ListPresetsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPresetsError {
    fn from(err: CredentialsError) -> ListPresetsError {
        ListPresetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPresetsError {
    fn from(err: HttpDispatchError) -> ListPresetsError {
        ListPresetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPresetsError {
    fn from(err: io::Error) -> ListPresetsError {
        ListPresetsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPresetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPresetsError {
    fn description(&self) -> &str {
        match *self {
            ListPresetsError::BadRequest(ref cause) => cause,
            ListPresetsError::Conflict(ref cause) => cause,
            ListPresetsError::Forbidden(ref cause) => cause,
            ListPresetsError::InternalServerError(ref cause) => cause,
            ListPresetsError::NotFound(ref cause) => cause,
            ListPresetsError::TooManyRequests(ref cause) => cause,
            ListPresetsError::Validation(ref cause) => cause,
            ListPresetsError::Credentials(ref err) => err.description(),
            ListPresetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPresetsError::ParseError(ref cause) => cause,
            ListPresetsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl ListQueuesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListQueuesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListQueuesError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return ListQueuesError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListQueuesError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListQueuesError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return ListQueuesError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListQueuesError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListQueuesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListQueuesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListQueuesError {
    fn from(err: serde_json::error::Error) -> ListQueuesError {
        ListQueuesError::ParseError(err.description().to_string())
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
            ListQueuesError::BadRequest(ref cause) => cause,
            ListQueuesError::Conflict(ref cause) => cause,
            ListQueuesError::Forbidden(ref cause) => cause,
            ListQueuesError::InternalServerError(ref cause) => cause,
            ListQueuesError::NotFound(ref cause) => cause,
            ListQueuesError::TooManyRequests(ref cause) => cause,
            ListQueuesError::Validation(ref cause) => cause,
            ListQueuesError::Credentials(ref err) => err.description(),
            ListQueuesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListQueuesError::ParseError(ref cause) => cause,
            ListQueuesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl ListTagsForResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return ListTagsForResourceError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return ListTagsForResourceError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return ListTagsForResourceError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return ListTagsForResourceError::InternalServerError(String::from(
                        error_message,
                    ));
                }
                "NotFoundException" => {
                    return ListTagsForResourceError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListTagsForResourceError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListTagsForResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListTagsForResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => cause,
            ListTagsForResourceError::Conflict(ref cause) => cause,
            ListTagsForResourceError::Forbidden(ref cause) => cause,
            ListTagsForResourceError::InternalServerError(ref cause) => cause,
            ListTagsForResourceError::NotFound(ref cause) => cause,
            ListTagsForResourceError::TooManyRequests(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::ParseError(ref cause) => cause,
            ListTagsForResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl TagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return TagResourceError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return TagResourceError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return TagResourceError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return TagResourceError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return TagResourceError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return TagResourceError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::BadRequest(ref cause) => cause,
            TagResourceError::Conflict(ref cause) => cause,
            TagResourceError::Forbidden(ref cause) => cause,
            TagResourceError::InternalServerError(ref cause) => cause,
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::TooManyRequests(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl UntagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UntagResourceError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return UntagResourceError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UntagResourceError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UntagResourceError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return UntagResourceError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UntagResourceError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::BadRequest(ref cause) => cause,
            UntagResourceError::Conflict(ref cause) => cause,
            UntagResourceError::Forbidden(ref cause) => cause,
            UntagResourceError::InternalServerError(ref cause) => cause,
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::TooManyRequests(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateJobTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl UpdateJobTemplateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateJobTemplateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateJobTemplateError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return UpdateJobTemplateError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UpdateJobTemplateError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateJobTemplateError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateJobTemplateError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateJobTemplateError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateJobTemplateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateJobTemplateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateJobTemplateError {
    fn from(err: serde_json::error::Error) -> UpdateJobTemplateError {
        UpdateJobTemplateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateJobTemplateError {
    fn from(err: CredentialsError) -> UpdateJobTemplateError {
        UpdateJobTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobTemplateError {
    fn from(err: HttpDispatchError) -> UpdateJobTemplateError {
        UpdateJobTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateJobTemplateError {
    fn from(err: io::Error) -> UpdateJobTemplateError {
        UpdateJobTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateJobTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobTemplateError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobTemplateError::BadRequest(ref cause) => cause,
            UpdateJobTemplateError::Conflict(ref cause) => cause,
            UpdateJobTemplateError::Forbidden(ref cause) => cause,
            UpdateJobTemplateError::InternalServerError(ref cause) => cause,
            UpdateJobTemplateError::NotFound(ref cause) => cause,
            UpdateJobTemplateError::TooManyRequests(ref cause) => cause,
            UpdateJobTemplateError::Validation(ref cause) => cause,
            UpdateJobTemplateError::Credentials(ref err) => err.description(),
            UpdateJobTemplateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateJobTemplateError::ParseError(ref cause) => cause,
            UpdateJobTemplateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdatePreset
#[derive(Debug, PartialEq)]
pub enum UpdatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl UpdatePresetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePresetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdatePresetError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return UpdatePresetError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UpdatePresetError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdatePresetError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdatePresetError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdatePresetError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdatePresetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdatePresetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePresetError {
    fn from(err: serde_json::error::Error) -> UpdatePresetError {
        UpdatePresetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePresetError {
    fn from(err: CredentialsError) -> UpdatePresetError {
        UpdatePresetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePresetError {
    fn from(err: HttpDispatchError) -> UpdatePresetError {
        UpdatePresetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePresetError {
    fn from(err: io::Error) -> UpdatePresetError {
        UpdatePresetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdatePresetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePresetError {
    fn description(&self) -> &str {
        match *self {
            UpdatePresetError::BadRequest(ref cause) => cause,
            UpdatePresetError::Conflict(ref cause) => cause,
            UpdatePresetError::Forbidden(ref cause) => cause,
            UpdatePresetError::InternalServerError(ref cause) => cause,
            UpdatePresetError::NotFound(ref cause) => cause,
            UpdatePresetError::TooManyRequests(ref cause) => cause,
            UpdatePresetError::Validation(ref cause) => cause,
            UpdatePresetError::Credentials(ref err) => err.description(),
            UpdatePresetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePresetError::ParseError(ref cause) => cause,
            UpdatePresetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateQueue
#[derive(Debug, PartialEq)]
pub enum UpdateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service could not complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and cannot fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested does not exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
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

impl UpdateQueueError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateQueueError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return UpdateQueueError::BadRequest(String::from(error_message));
                }
                "ConflictException" => {
                    return UpdateQueueError::Conflict(String::from(error_message));
                }
                "ForbiddenException" => {
                    return UpdateQueueError::Forbidden(String::from(error_message));
                }
                "InternalServerErrorException" => {
                    return UpdateQueueError::InternalServerError(String::from(error_message));
                }
                "NotFoundException" => {
                    return UpdateQueueError::NotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateQueueError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateQueueError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateQueueError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateQueueError {
    fn from(err: serde_json::error::Error) -> UpdateQueueError {
        UpdateQueueError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateQueueError {
    fn from(err: CredentialsError) -> UpdateQueueError {
        UpdateQueueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateQueueError {
    fn from(err: HttpDispatchError) -> UpdateQueueError {
        UpdateQueueError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateQueueError {
    fn from(err: io::Error) -> UpdateQueueError {
        UpdateQueueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateQueueError {
    fn description(&self) -> &str {
        match *self {
            UpdateQueueError::BadRequest(ref cause) => cause,
            UpdateQueueError::Conflict(ref cause) => cause,
            UpdateQueueError::Forbidden(ref cause) => cause,
            UpdateQueueError::InternalServerError(ref cause) => cause,
            UpdateQueueError::NotFound(ref cause) => cause,
            UpdateQueueError::TooManyRequests(ref cause) => cause,
            UpdateQueueError::Validation(ref cause) => cause,
            UpdateQueueError::Credentials(ref err) => err.description(),
            UpdateQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateQueueError::ParseError(ref cause) => cause,
            UpdateQueueError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the MediaConvert API. MediaConvert clients implement this trait.
pub trait MediaConvert {
    /// <p>Permanently remove a job from a queue. Once you have canceled a job, you can&#39;t start it again. You can&#39;t delete a running job.</p>
    fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> RusotoFuture<CancelJobResponse, CancelJobError>;

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError>;

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> RusotoFuture<CreateJobTemplateResponse, CreateJobTemplateError>;

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> RusotoFuture<CreatePresetResponse, CreatePresetError>;

    /// <p>Create a new transcoding queue. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> RusotoFuture<CreateQueueResponse, CreateQueueError>;

    /// <p>Permanently delete a job template you have created.</p>
    fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> RusotoFuture<DeleteJobTemplateResponse, DeleteJobTemplateError>;

    /// <p>Permanently delete a preset you have created.</p>
    fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> RusotoFuture<DeletePresetResponse, DeletePresetError>;

    /// <p>Permanently delete a queue you have created.</p>
    fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> RusotoFuture<DeleteQueueResponse, DeleteQueueError>;

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> RusotoFuture<DescribeEndpointsResponse, DescribeEndpointsError>;

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError>;

    /// <p>Retrieve the JSON for a specific job template.</p>
    fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> RusotoFuture<GetJobTemplateResponse, GetJobTemplateError>;

    /// <p>Retrieve the JSON for a specific preset.</p>
    fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> RusotoFuture<GetPresetResponse, GetPresetError>;

    /// <p>Retrieve the JSON for a specific queue.</p>
    fn get_queue(&self, input: GetQueueRequest) -> RusotoFuture<GetQueueResponse, GetQueueError>;

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> RusotoFuture<ListJobTemplatesResponse, ListJobTemplatesError>;

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError>;

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> RusotoFuture<ListPresetsResponse, ListPresetsError>;

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> RusotoFuture<ListQueuesResponse, ListQueuesError>;

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Tag a MediaConvert queue, preset, or job template. For information about these resource types, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Untag a MediaConvert queue, preset, or job template. For information about these resource types, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Modify one of your existing job templates.</p>
    fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> RusotoFuture<UpdateJobTemplateResponse, UpdateJobTemplateError>;

    /// <p>Modify one of your existing presets.</p>
    fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> RusotoFuture<UpdatePresetResponse, UpdatePresetError>;

    /// <p>Modify one of your existing queues.</p>
    fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> RusotoFuture<UpdateQueueResponse, UpdateQueueError>;
}
/// A client for the MediaConvert API.
#[derive(Clone)]
pub struct MediaConvertClient {
    client: Client,
    region: region::Region,
}

impl MediaConvertClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaConvertClient {
        MediaConvertClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaConvertClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MediaConvertClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl MediaConvert for MediaConvertClient {
    /// <p>Permanently remove a job from a queue. Once you have canceled a job, you can&#39;t start it again. You can&#39;t delete a running job.</p>
    fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> RusotoFuture<CancelJobResponse, CancelJobError> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CancelJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> RusotoFuture<CreateJobResponse, CreateJobError> {
        let request_uri = "/2017-08-29/jobs";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> RusotoFuture<CreateJobTemplateResponse, CreateJobTemplateError> {
        let request_uri = "/2017-08-29/jobTemplates";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateJobTemplateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateJobTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> RusotoFuture<CreatePresetResponse, CreatePresetError> {
        let request_uri = "/2017-08-29/presets";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreatePresetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePresetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Create a new transcoding queue. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> RusotoFuture<CreateQueueResponse, CreateQueueError> {
        let request_uri = "/2017-08-29/queues";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateQueueError::from_response(response))),
                )
            }
        })
    }

    /// <p>Permanently delete a job template you have created.</p>
    fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> RusotoFuture<DeleteJobTemplateResponse, DeleteJobTemplateError> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteJobTemplateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteJobTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Permanently delete a preset you have created.</p>
    fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> RusotoFuture<DeletePresetResponse, DeletePresetError> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeletePresetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePresetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Permanently delete a queue you have created.</p>
    fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> RusotoFuture<DeleteQueueResponse, DeleteQueueError> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteQueueError::from_response(response))),
                )
            }
        })
    }

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> RusotoFuture<DescribeEndpointsResponse, DescribeEndpointsError> {
        let request_uri = "/2017-08-29/endpoints";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeEndpointsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResponse, GetJobError> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetJobResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve the JSON for a specific job template.</p>
    fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> RusotoFuture<GetJobTemplateResponse, GetJobTemplateError> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetJobTemplateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetJobTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve the JSON for a specific preset.</p>
    fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> RusotoFuture<GetPresetResponse, GetPresetError> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetPresetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPresetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve the JSON for a specific queue.</p>
    fn get_queue(&self, input: GetQueueRequest) -> RusotoFuture<GetQueueResponse, GetQueueError> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetQueueError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> RusotoFuture<ListJobTemplatesResponse, ListJobTemplatesError> {
        let request_uri = "/2017-08-29/jobTemplates";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.category {
            params.put("category", x);
        }
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListJobTemplatesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListJobTemplatesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResponse, ListJobsError> {
        let request_uri = "/2017-08-29/jobs";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        if let Some(ref x) = input.queue {
            params.put("queue", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListJobsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> RusotoFuture<ListPresetsResponse, ListPresetsError> {
        let request_uri = "/2017-08-29/presets";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.category {
            params.put("category", x);
        }
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListPresetsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPresetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> RusotoFuture<ListQueuesResponse, ListQueuesError> {
        let request_uri = "/2017-08-29/queues";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListQueuesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListQueuesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/2017-08-29/tags/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListTagsForResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Tag a MediaConvert queue, preset, or job template. For information about these resource types, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = "/2017-08-29/tags";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Untag a MediaConvert queue, preset, or job template. For information about these resource types, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = "/2017-08-29/tags";

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagResourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modify one of your existing job templates.</p>
    fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> RusotoFuture<UpdateJobTemplateResponse, UpdateJobTemplateError> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateJobTemplateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateJobTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modify one of your existing presets.</p>
    fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> RusotoFuture<UpdatePresetResponse, UpdatePresetError> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdatePresetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePresetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modify one of your existing queues.</p>
    fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> RusotoFuture<UpdateQueueResponse, UpdateQueueError> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateQueueResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateQueueError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
