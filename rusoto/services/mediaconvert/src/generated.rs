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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AAC. The service accepts one of two mutually exclusive groups of AAC settings--VBR and CBR. To select one of these modes, set the value of Bitrate control mode (rateControlMode) to &quot;VBR&quot; or &quot;CBR&quot;.  In VBR mode, you control the audio quality with the setting VBR quality (vbrQuality). In CBR mode, you use the setting Bitrate (bitrate). Defaults and valid values depend on the rate control mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AacSettings {
    /// <p>Choose BROADCASTER<em>MIXED</em>AD when the input contains pre-mixed main audio + audio description (AD) as a stereo pair. The value for AudioType will be set to 3, which signals to downstream systems that this stream contains &quot;broadcaster mixed AD&quot;. Note that the input received by the encoder must contain pre-mixed audio; the encoder does not perform the mixing. When you choose BROADCASTER<em>MIXED</em>AD, the encoder ignores any values you provide in AudioType and  FollowInputAudioType. Choose NORMAL when the input does not contain pre-mixed audio + audio description (AD). In this case, the encoder will use any values you provide for AudioType and FollowInputAudioType.</p>
    #[serde(rename = "AudioDescriptionBroadcasterMix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_broadcaster_mix: Option<String>,
    /// <p>Specify the average bitrate in bits per second. The set of valid values for this setting is: 6000, 8000, 10000, 12000, 14000, 16000, 20000, 24000, 28000, 32000, 40000, 48000, 56000, 64000, 80000, 96000, 112000, 128000, 160000, 192000, 224000, 256000, 288000, 320000, 384000, 448000, 512000, 576000, 640000, 768000, 896000, 1024000. The value you set is also constrained by the values that you choose for Profile (codecProfile), Bitrate control mode (codingMode), and Sample rate (sampleRate). Default values depend on Bitrate control mode and Profile.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>AAC Profile.</p>
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Mono (Audio Description), Mono, Stereo, or 5.1 channel layout. Valid values depend on rate control mode and profile. &quot;1.0 - Audio Description (Receiver Mix)&quot; setting receives a stereo description plus control track and emits a mono AAC encode of the description track, with control data emitted in the PES header as per ETSI TS 101 154 Annex E.</p>
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Rate Control Mode.</p>
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Enables LATM/LOAS AAC output. Note that if you use LATM/LOAS AAC in an output, you must choose &quot;No container&quot; for the output container.</p>
    #[serde(rename = "RawFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<String>,
    /// <p>Sample rate in Hz. Valid values depend on rate control mode and profile.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Use MPEG-2 AAC instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.</p>
    #[serde(rename = "Specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    /// <p>VBR Quality Level - Only used if rate<em>control</em>mode is VBR.</p>
    #[serde(rename = "VbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AC3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ac3Settings {
    /// <p>Specify the average bitrate in bits per second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the AC-3 stream that the encoder emits. For more information about the AC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital coding mode. Determines number of channels.</p>
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital, dialnorm will be passed through.</p>
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>If set to FILM_STANDARD, adds dynamic range compression signaling to the output bitstream as defined in the Dolby Digital specification.</p>
    #[serde(rename = "DynamicRangeCompressionProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_profile: Option<String>,
    /// <p>Applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid with 3<em>2</em>LFE coding mode.</p>
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>When set to FOLLOW_INPUT, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccelerationSettings {
    /// <p>Specify the conditions when the service will run your job with accelerated transcoding.</p>
    #[serde(rename = "Mode")]
    pub mode: String,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AIFF.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiffSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "BitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Valid values are 1 and even numbers up to 64. For example, 1, 2, 4, 6, and so on, up to 64.</p>
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
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Specifies the 608 channel number in the ancillary data track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "SourceAncillaryChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i64>,
    /// <p>By default, the service terminates any unterminated captions at the end of each input. If you want the caption to continue onto your next input, disable this setting.</p>
    #[serde(rename = "TerminateCaptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_captions: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateCertificateRequest {
    /// <p>The ARN of the ACM certificate that you want to associate with your MediaConvert resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateCertificateResponse {}

/// <p>Audio codec settings (CodecSettings) under (AudioDescriptions) contains the group of settings related to audio encoding. The settings in this group vary depending on the value that you choose for Audio codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * AAC, AacSettings * MP2, Mp2Settings * MP3, Mp3Settings * WAV, WavSettings * AIFF, AiffSettings * AC3, Ac3Settings * EAC3, Eac3Settings * EAC3_ATMOS, Eac3AtmosSettings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioCodecSettings {
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AAC. The service accepts one of two mutually exclusive groups of AAC settings--VBR and CBR. To select one of these modes, set the value of Bitrate control mode (rateControlMode) to &quot;VBR&quot; or &quot;CBR&quot;.  In VBR mode, you control the audio quality with the setting VBR quality (vbrQuality). In CBR mode, you use the setting Bitrate (bitrate). Defaults and valid values depend on the rate control mode.</p>
    #[serde(rename = "AacSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AC3.</p>
    #[serde(rename = "Ac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_3_settings: Option<Ac3Settings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AIFF.</p>
    #[serde(rename = "AiffSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aiff_settings: Option<AiffSettings>,
    /// <p>Type of Audio codec.</p>
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3_ATMOS.</p>
    #[serde(rename = "Eac3AtmosSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_atmos_settings: Option<Eac3AtmosSettings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3.</p>
    #[serde(rename = "Eac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_settings: Option<Eac3Settings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value MP2.</p>
    #[serde(rename = "Mp2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_2_settings: Option<Mp2Settings>,
    /// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value MP3.</p>
    #[serde(rename = "Mp3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_3_settings: Option<Mp3Settings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value WAV.</p>
    #[serde(rename = "WavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

/// <p>Description of audio output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioDescription {
    /// <p>Advanced audio normalization settings. Ignore these settings unless you need to comply with a loudness standard.</p>
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
    /// <p>When set to FOLLOW<em>INPUT, if the input contains an ISO 639 audio</em>type, then that value is passed through to the output. If the input contains no ISO 639 audio<em>type, the value in Audio Type is included in the output. Otherwise the value in Audio Type is included in the output. Note that this field and audioType are both ignored if audioDescriptionBroadcasterMix is set to BROADCASTER</em>MIXED_AD.</p>
    #[serde(rename = "AudioTypeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<String>,
    /// <p>Audio codec settings (CodecSettings) under (AudioDescriptions) contains the group of settings related to audio encoding. The settings in this group vary depending on the value that you choose for Audio codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * AAC, AacSettings * MP2, Mp2Settings * MP3, Mp3Settings * WAV, WavSettings * AIFF, AiffSettings * AC3, Ac3Settings * EAC3, Eac3Settings * EAC3_ATMOS, Eac3AtmosSettings</p>
    #[serde(rename = "CodecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<AudioCodecSettings>,
    /// <p>Specify the language for this audio output track. The service puts this language code into your output audio track when you set Language code control (AudioLanguageCodeControl) to Use configured (USE<em>CONFIGURED). The service also uses your specified custom language code when you set Language code control (AudioLanguageCodeControl) to Follow input (FOLLOW</em>INPUT), but your input file doesn&#39;t specify a language code. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Indicates the language of the audio output track. The ISO 639 language specified in the &#39;Language Code&#39; drop down will be used when &#39;Follow Input Language Code&#39; is not selected or when &#39;Follow Input Language Code&#39; is selected but there is no ISO 639 language code specified by the input.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify which source for language code takes precedence for this audio track. When you choose Follow input (FOLLOW<em>INPUT), the service uses the language code from the input track if it&#39;s present. If there&#39;s no languge code on the input track, the service uses the code that you specify in the setting Language code (languageCode or customLanguageCode). When you choose Use configured (USE</em>CONFIGURED), the service uses the language code that you specify.</p>
    #[serde(rename = "LanguageCodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<String>,
    /// <p>Advanced audio remixing settings.</p>
    #[serde(rename = "RemixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    /// <p>Specify a label for this output audio stream. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Advanced audio normalization settings. Ignore these settings unless you need to comply with a loudness standard.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNormalizationSettings {
    /// <p>Choose one of the following audio normalization algorithms: ITU-R BS.1770-1: Ungated loudness. A measurement of ungated average loudness for an entire piece of content, suitable for measurement of short-form content under ATSC recommendation A/85. Supports up to 5.1 audio channels. ITU-R BS.1770-2: Gated loudness. A measurement of gated average loudness compliant with the requirements of EBU-R128. Supports up to 5.1 audio channels. ITU-R BS.1770-3: Modified peak. The same loudness measurement algorithm as 1770-2, with an updated true peak measurement. ITU-R BS.1770-4: Higher channel count. Allows for more audio channels than the other algorithms, including configurations such as 7.1.</p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>When enabled the output audio is corrected using the chosen algorithm. If disabled, the audio will be measured but not adjusted.</p>
    #[serde(rename = "AlgorithmControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<String>,
    /// <p>Content measuring above this level will be corrected to the target level. Content measuring below this level will not be corrected. Gating only applies when not using real<em>time</em>correction.</p>
    #[serde(rename = "CorrectionGateLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_gate_level: Option<i64>,
    /// <p>If set to LOG, log each output&#39;s audio track loudness to a CSV file.</p>
    #[serde(rename = "LoudnessLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness_logging: Option<String>,
    /// <p>If set to TRUE_PEAK, calculate and log the TruePeak for each output&#39;s audio track loudness.</p>
    #[serde(rename = "PeakCalculation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_calculation: Option<String>,
    /// <p>When you use Audio normalization (AudioNormalizationSettings), optionally use this setting to specify a target loudness. If you don&#39;t specify a value here, the encoder chooses a value for you, based on the algorithm that you choose for Algorithm (algorithm). If you choose algorithm 1770-1, the encoder will choose -24 LKFS; otherwise, the encoder will choose -23 LKFS.</p>
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
    /// <p>Enable this setting on one audio selector to set it as the default for the job. The service uses this default for outputs where it can&#39;t find the specified input audio. If you don&#39;t set a default, those outputs have no audio.</p>
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
    /// <p>Specifies the type of the audio selector.</p>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_names: Option<Vec<String>>,
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
    /// <p>If no explicit x<em>position or y</em>position is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>Provide the font script, using an ISO 15924 script code, if the LanguageCode is not sufficient for determining the script type. Where LanguageCode or CustomLanguageCode is sufficient, use &quot;AUTOMATIC&quot; or leave unset. This is used to help determine the appropriate font for rendering burn-in captions.</p>
    #[serde(rename = "FontScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_script: Option<String>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
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
    /// <p>Only applies to jobs with input captions in Teletext or STL formats. Specify whether the spacing between letters in your captions is set by the captions grid or varies depending on letter width. Choose fixed grid to conform to the spacing specified in the captions file more accurately. Choose proportional to make the text easier to read if the captions are closed caption.</p>
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRequest {
    /// <p>The Job ID of the job to be cancelled.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>Description of Caption output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDescription {
    /// <p>Specifies which &quot;Caption Selector&quot;:#inputs-caption_selector to use from each input when generating captions. The name should be of the format &quot;Caption Selector <N>&quot;, which denotes that the Nth Caption Selector will be used from each input.</p>
    #[serde(rename = "CaptionSelectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selector_name: Option<String>,
    /// <p>Specify the language for this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information when automatically selecting the font script for rendering the captions text. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Specific settings required by destination type. Note that burnin<em>destination</em>settings are not available if the source of the caption data is Embedded or Teletext.</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    /// <p>Specify the language of this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information to choose the font language for rendering the captions text.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify a label for this set of output captions. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Caption Description for preset</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDescriptionPreset {
    /// <p>Specify the language for this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information when automatically selecting the font script for rendering the captions text. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Specific settings required by destination type. Note that burnin<em>destination</em>settings are not available if the source of the caption data is Embedded or Teletext.</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    /// <p>Specify the language of this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information to choose the font language for rendering the captions text.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify a label for this set of output captions. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "LanguageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Specific settings required by destination type. Note that burnin<em>destination</em>settings are not available if the source of the caption data is Embedded or Teletext.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionDestinationSettings {
    /// <p>Burn-In Destination Settings.</p>
    #[serde(rename = "BurninDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnin_destination_settings: Option<BurninDestinationSettings>,
    /// <p>Specify the format for this set of captions on this output. The default format is embedded without SCTE-20. Other options are embedded with SCTE-20, burn-in, DVB-sub, IMSC, SCC, SRT, teletext, TTML, and web-VTT. If you are using SCTE-20, choose SCTE-20 plus embedded (SCTE20<em>PLUS</em>EMBEDDED) to create an output that complies with the SCTE-43 spec. To create a non-compliant output where the embedded captions come first, choose Embedded plus SCTE-20 (EMBEDDED<em>PLUS</em>SCTE20).</p>
    #[serde(rename = "DestinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    /// <p>DVB-Sub Destination Settings</p>
    #[serde(rename = "DvbSubDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    /// <p>Settings specific to embedded/ancillary caption outputs, including 608/708 Channel destination number.</p>
    #[serde(rename = "EmbeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,
    /// <p>Settings specific to IMSC caption outputs.</p>
    #[serde(rename = "ImscDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsc_destination_settings: Option<ImscDestinationSettings>,
    /// <p>Settings for SCC caption output.</p>
    #[serde(rename = "SccDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_destination_settings: Option<SccDestinationSettings>,
    /// <p>Settings for Teletext caption output</p>
    #[serde(rename = "TeletextDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    /// <p>Settings specific to TTML caption outputs, including Pass style information (TtmlStylePassthrough).</p>
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
    /// <p>If your input captions are SCC, TTML, STL, SMI, SRT, or IMSC in an xml file, specify the URI of the input captions source file. If your input captions are IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
    #[serde(rename = "SourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_settings: Option<CaptionSourceSettings>,
}

/// <p>If your input captions are SCC, TTML, STL, SMI, SRT, or IMSC in an xml file, specify the URI of the input captions source file. If your input captions are IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionSourceSettings {
    /// <p>Settings for ancillary captions source.</p>
    #[serde(rename = "AncillarySourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,
    /// <p>DVB Sub Source Settings</p>
    #[serde(rename = "DvbSubSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    /// <p>Settings for embedded captions Source</p>
    #[serde(rename = "EmbeddedSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    /// <p>If your input captions are SCC, SMI, SRT, STL, TTML, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
    #[serde(rename = "FileSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_source_settings: Option<FileSourceSettings>,
    /// <p>Use Source (SourceType) to identify the format of your input captions.  The service cannot auto-detect caption format.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>Settings specific to Teletext caption sources, including Page number.</p>
    #[serde(rename = "TeletextSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
    /// <p>Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.</p>
    #[serde(rename = "TrackSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_source_settings: Option<TrackSourceSettings>,
}

/// <p>Channel mapping (ChannelMapping) contains the group of fields that hold the remixing value for each channel. Units are in dB. Acceptable values are within the range from -60 (mute) through 6. A setting of 0 passes the input channel unchanged to the output channel (no attenuation or amplification).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelMapping {
    /// <p>List of output channels</p>
    #[serde(rename = "OutputChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_channels: Option<Vec<OutputChannelMapping>>,
}

/// <p>Specify the details for each pair of HLS and DASH additional manifests that you want the service to generate for this CMAF output group. Each pair of manifests can reference a different subset of outputs in the group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your HLS group is film-name.m3u8. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.m3u8. For HLS output groups, specify a manifestNameModifier that is different from the nameModifier of the output. The service uses the output name modifier to create unique names for the individual variant manifests.</p>
    #[serde(rename = "ManifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "SelectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Settings for CMAF encryption</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafEncryptionSettings {
    /// <p>This is a 128-bit, 16-byte hex value represented by a 32-character text string. If this parameter is not set then the Initialization Vector will follow the segment number by default.</p>
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>Specify the encryption scheme that you want the service to use when encrypting your CMAF segments. Choose AES-CBC subsample (SAMPLE-AES) or AES_CTR (AES-CTR).</p>
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>When you use DRM with CMAF outputs, choose whether the service writes the 128-bit encryption initialization vector in the HLS and DASH manifests.</p>
    #[serde(rename = "InitializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    /// <p>If your output group type is CMAF, use these settings when doing DRM encryption with a SPEKE-compliant key provider. If your output group type is HLS, DASH, or Microsoft Smooth, use the SpekeKeyProvider settings instead.</p>
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProviderCmaf>,
    /// <p>Use these settings to set up encryption with a static key provider.</p>
    #[serde(rename = "StaticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    /// <p>Specify whether your DRM encryption key is static or from a key provider that follows the SPEKE standard. For more information about SPEKE, see https://docs.aws.amazon.com/speke/latest/documentation/what-is-speke.html.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to CMAF<em>GROUP</em>SETTINGS. Each output in a CMAF Output Group may only contain a single video, audio, or caption output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmafGroupSettings {
    /// <p>By default, the service creates one top-level .m3u8 HLS manifest and one top -level .mpd DASH manifest for each CMAF output group in your job. These default manifests reference every output in the output group. To create additional top-level manifests that reference a subset of the outputs in the output group, specify a list of them here. For each additional manifest that you specify, the service creates one HLS manifest and one DASH manifest.</p>
    #[serde(rename = "AdditionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<CmafAdditionalManifest>>,
    /// <p>A partial URI prefix that will be put in the manifest file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>When set to ENABLED, sets #EXT-X-ALLOW-CACHE:no tag, which prevents client from saving media segments for later replay.</p>
    #[serde(rename = "ClientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    /// <p>Specification to use (RFC-6381 or the default RFC-4281) during m3u8 playlist generation.</p>
    #[serde(rename = "CodecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "FragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>When set to GZIP, compresses HLS playlist.</p>
    #[serde(rename = "ManifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    /// <p>Indicates whether the output manifest should use floating point values for segment duration.</p>
    #[serde(rename = "ManifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "MinBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    /// <p>Keep this setting at the default value of 0, unless you are troubleshooting a problem with how devices play back the end of your video asset. If you know that player devices are hanging on the final segment of your video because the length of your final segment is too short, use this setting to specify a minimum final segment length, in seconds. Choose a value that is greater than or equal to 1 and less than your segment length. When you specify a value for this setting, the encoder will combine any final segment that is shorter than the length that you specify with the previous segment. For example, your segment length is 3 seconds and your final segment is .5 seconds without a minimum final segment length; when you set the minimum final segment length to 1, your final segment is 3.5 seconds.</p>
    #[serde(rename = "MinFinalSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_final_segment_length: Option<f64>,
    /// <p>Specify whether your DASH profile is on-demand or main. When you choose Main profile (MAIN<em>PROFILE), the service signals  urn:mpeg:dash:profile:isoff-main:2011 in your .mpd DASH manifest. When you choose On-demand (ON</em>DEMAND<em>PROFILE), the service signals urn:mpeg:dash:profile:isoff-on-demand:2011 in your .mpd. When you choose On-demand, you must also set the output group setting Segment control (SegmentControl) to Single file (SINGLE</em>FILE).</p>
    #[serde(rename = "MpdProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_profile: Option<String>,
    /// <p>When set to SINGLE<em>FILE, a single output file is generated, which is internally segmented using the Fragment Length and Segment Length. When set to SEGMENTED</em>FILES, separate segment files will be created.</p>
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Use this setting to specify the length, in seconds, of each individual CMAF segment. This value applies to the whole package; that is, to every output in the output group. Note that segments end on the first keyframe after this number of seconds, so the actual segment length might be slightly longer. If you set Segment control (CmafSegmentControl) to single file, the service puts the content of each output in a single file that has metadata that marks these segments. If you set it to segmented files, the service creates multiple files for each output, each with the content of one segment.</p>
    #[serde(rename = "SegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>Include or exclude RESOLUTION attribute for video in EXT-X-STREAM-INF tag of variant manifest.</p>
    #[serde(rename = "StreamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    /// <p>When set to ENABLED, a DASH MPD manifest will be generated for this output.</p>
    #[serde(rename = "WriteDashManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_dash_manifest: Option<String>,
    /// <p>When set to ENABLED, an Apple HLS manifest will be generated for this output.</p>
    #[serde(rename = "WriteHlsManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_hls_manifest: Option<String>,
    /// <p>When you enable Precise segment duration in DASH manifests (writeSegmentTimelineInRepresentation), your DASH manifest shows precise segment durations. The segment duration information appears inside the SegmentTimeline element, inside SegmentTemplate at the Representation level. When this feature isn&#39;t enabled, the segment durations in your DASH manifest are approximate. The segment duration information appears in the duration attribute of the SegmentTemplate element.</p>
    #[serde(rename = "WriteSegmentTimelineInRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_segment_timeline_in_representation: Option<String>,
}

/// <p>Settings for MP4 segments in CMAF</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmfcSettings {
    /// <p>Use this setting only when you specify SCTE-35 markers from ESAM. Choose INSERT to put SCTE-35 markers in this output at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "Scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<String>,
    /// <p>Ignore this setting unless you have SCTE-35 markers in your input video file. Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want those SCTE-35 markers in this output.</p>
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
}

/// <p>Settings for color correction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorCorrector {
    /// <p>Brightness level.</p>
    #[serde(rename = "Brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i64>,
    /// <p>Specify the color space you want for this output. The service supports conversion between HDR formats, between SDR formats, and from SDR to HDR. The service doesn&#39;t support conversion from HDR to SDR. SDR to HDR conversion doesn&#39;t upgrade the dynamic range. The converted video has an HDR format, but visually appears the same as an unconverted output.</p>
    #[serde(rename = "ColorSpaceConversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_conversion: Option<String>,
    /// <p>Contrast level.</p>
    #[serde(rename = "Contrast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<i64>,
    /// <p>Use these settings when you convert to the HDR 10 color space. Specify the SMPTE ST 2086 Mastering Display Color Volume static metadata that you want signaled in the output. These values don&#39;t affect the pixel values that are encoded in the video stream. They are intended to help the downstream video player display content in a way that reflects the intentions of the the content creator. When you set Color space conversion (ColorSpaceConversion) to HDR 10 (FORCE_HDR10), these settings are required. You must set values for Max frame average light level (maxFrameAverageLightLevel) and Max content light level (maxContentLightLevel); these settings don&#39;t have a default value. The default values for the other HDR 10 metadata settings are defined by the P3D65 color space. For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
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
    /// <p>Settings for MP4 segments in CMAF</p>
    #[serde(rename = "CmfcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmfc_settings: Option<CmfcSettings>,
    /// <p>Container for this output. Some containers require a container settings object. If not specified, the default object will be created.</p>
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// <p>Settings for F4v container</p>
    #[serde(rename = "F4vSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_4v_settings: Option<F4vSettings>,
    /// <p>MPEG-2 TS container settings. These apply to outputs in a File output group when the output&#39;s container (ContainerType) is MPEG-2 Transport Stream (M2TS). In these assets, data is organized by the program map table (PMT). Each transport stream program contains subsets of data, including audio, video, and metadata. Each of these subsets of data has a numerical label called a packet identifier (PID). Each transport stream program corresponds to one MediaConvert output. The PMT lists the types of data in a program along with their PID. Downstream systems and players use the program map table to look up the PID for each type of data it accesses and then uses the PIDs to locate specific data within the asset.</p>
    #[serde(rename = "M2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_2ts_settings: Option<M2tsSettings>,
    /// <p>Settings for TS segments in HLS</p>
    #[serde(rename = "M3u8Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_3u_8_settings: Option<M3u8Settings>,
    /// <p>Settings for MOV Container.</p>
    #[serde(rename = "MovSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mov_settings: Option<MovSettings>,
    /// <p>Settings for MP4 container. You can create audio-only AAC outputs with this container.</p>
    #[serde(rename = "Mp4Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_settings: Option<Mp4Settings>,
    /// <p>Settings for MP4 segments in DASH</p>
    #[serde(rename = "MpdSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_settings: Option<MpdSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobRequest {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don&#39;t have an associated tag will appear in your billing report unsorted. If you don&#39;t choose a valid value for this field, your job outputs will appear on the billing report unsorted.</p>
    #[serde(rename = "BillingTagsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tags_source: Option<String>,
    /// <p>Idempotency token for CreateJob operation.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>When you create a job, you can either specify a job template or specify the transcoding settings individually</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>JobSettings contains all the transcode settings for a job.</p>
    #[serde(rename = "Settings")]
    pub settings: JobSettings,
    /// <p>Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.</p>
    #[serde(rename = "SimulateReservedQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulate_reserved_queue: Option<String>,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.</p>
    #[serde(rename = "UserMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobResponse {
    /// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobTemplateRequest {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
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
    /// <p>Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "Settings")]
    pub settings: JobTemplateSettings,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Settings for preset</p>
    #[serde(rename = "Settings")]
    pub settings: PresetSettings,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateQueueRequest {
    /// <p>Optional. A description of the queue that you are creating.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue that you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.</p>
    #[serde(rename = "PricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,
    /// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
    #[serde(rename = "ReservationPlanSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan_settings: Option<ReservationPlanSettings>,
    /// <p>Initial state of the queue. If you create a paused queue, then jobs in that queue won&#39;t begin.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Specify the details for each additional DASH manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your DASH group is film-name.mpd. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.mpd.</p>
    #[serde(rename = "ManifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "SelectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Specifies DRM settings for DASH outputs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashIsoEncryptionSettings {
    /// <p>This setting can improve the compatibility of your output with video players on obsolete devices. It applies only to DASH H.264 outputs with DRM encryption. Choose Unencrypted SEI (UNENCRYPTED<em>SEI) only to correct problems with playback on older devices. Otherwise, keep the default setting CENC v1 (CENC</em>V1). If you choose Unencrypted SEI, for that output, the service will exclude the access unit delimiter and will leave the SEI NAL units unencrypted.</p>
    #[serde(rename = "PlaybackDeviceCompatibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_device_compatibility: Option<String>,
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to DASH<em>ISO</em>GROUP_SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashIsoGroupSettings {
    /// <p>By default, the service creates one .mpd DASH manifest for each DASH ISO output group in your job. This default manifest references every output in the output group. To create additional DASH manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "AdditionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<DashAdditionalManifest>>,
    /// <p>A partial URI prefix that will be put in the manifest (.mpd) file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashIsoEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "FragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Supports HbbTV specification as indicated</p>
    #[serde(rename = "HbbtvCompliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hbbtv_compliance: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "MinBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    /// <p>Specify whether your DASH profile is on-demand or main. When you choose Main profile (MAIN<em>PROFILE), the service signals  urn:mpeg:dash:profile:isoff-main:2011 in your .mpd DASH manifest. When you choose On-demand (ON</em>DEMAND<em>PROFILE), the service signals urn:mpeg:dash:profile:isoff-on-demand:2011 in your .mpd. When you choose On-demand, you must also set the output group setting Segment control (SegmentControl) to Single file (SINGLE</em>FILE).</p>
    #[serde(rename = "MpdProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_profile: Option<String>,
    /// <p>When set to SINGLE<em>FILE, a single output file is generated, which is internally segmented using the Fragment Length and Segment Length. When set to SEGMENTED</em>FILES, separate segment files will be created.</p>
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of mpd segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer. When Emit Single File is checked, the segmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "SegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>If you get an HTTP error in the 400 range when you play back your DASH output, enable this setting and run your transcoding job again. When you enable this setting, the service writes precise segment durations in the DASH manifest. The segment duration information appears inside the SegmentTimeline element, inside SegmentTemplate at the Representation level. When you don&#39;t enable this setting, the service writes approximate segment durations in your DASH manifest.</p>
    #[serde(rename = "WriteSegmentTimelineInRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_segment_timeline_in_representation: Option<String>,
}

/// <p>Settings for deinterlacer</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deinterlacer {
    /// <p>Only applies when you set Deinterlacer (DeinterlaceMode) to Deinterlace (DEINTERLACE) or Adaptive (ADAPTIVE). Motion adaptive interpolate (INTERPOLATE) produces sharper pictures, while blend (BLEND) produces smoother motion. Use (INTERPOLATE<em>TICKER) OR (BLEND</em>TICKER) if your source file includes a ticker, such as a scrolling headline at the bottom of the frame.</p>
    #[serde(rename = "Algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <ul>
    /// <li>When set to NORMAL (default), the deinterlacer does not convert frames that are tagged  in metadata as progressive. It will only convert those that are tagged as some other type. - When set to FORCE<em>ALL</em>FRAMES, the deinterlacer converts every frame to progressive - even those that are already tagged as progressive. Turn Force mode on only if there is  a good chance that the metadata has tagged frames as progressive when they are not  progressive. Do not turn on otherwise; processing frames that are already progressive  into progressive will probably result in lower quality video.</li>
    /// </ul>
    #[serde(rename = "Control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>Use Deinterlacer (DeinterlaceMode) to choose how the service will do deinterlacing. Default is Deinterlace. - Deinterlace converts interlaced to progressive. - Inverse telecine converts Hard Telecine 29.97i to progressive 23.976p. - Adaptive auto-detects and converts to progressive.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJobTemplateRequest {
    /// <p>The name of the job template to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobTemplateResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePresetRequest {
    /// <p>The name of the preset to be deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePresetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteQueueRequest {
    /// <p>The name of the queue that you want to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteQueueResponse {}

/// <p>DescribeEndpointsRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointsRequest {
    /// <p>Optional. Max number of endpoints, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Optional field, defaults to DEFAULT. Specify DEFAULT for this operation to return your endpoints if any exist, or to create an endpoint for you and return it if one doesn&#39;t already exist. Specify GET_ONLY to return your endpoints if any exist, or an empty list if none exist.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of endpoints.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Settings associated with the destination. Will vary based on the type of destination</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationSettings {
    /// <p>Settings associated with S3 destination</p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3DestinationSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateCertificateRequest {
    /// <p>The ARN of the ACM certificate that you want to disassociate from your MediaConvert resource.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateCertificateResponse {}

/// <p>Settings for Dolby Vision</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DolbyVision {
    /// <p>Use these settings when you set DolbyVisionLevel6Mode to SPECIFY to override the MaxCLL and MaxFALL values in your input with new values.</p>
    #[serde(rename = "L6Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l6_metadata: Option<DolbyVisionLevel6Metadata>,
    /// <p>Use Dolby Vision Mode to choose how the service will handle Dolby Vision MaxCLL and MaxFALL properies.</p>
    #[serde(rename = "L6Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l6_mode: Option<String>,
    /// <p>In the current MediaConvert implementation, the Dolby Vision profile is always 5 (PROFILE_5). Therefore, all of your inputs must contain Dolby Vision frame interleaved data.</p>
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

/// <p>Use these settings when you set DolbyVisionLevel6Mode to SPECIFY to override the MaxCLL and MaxFALL values in your input with new values.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DolbyVisionLevel6Metadata {
    /// <p>Maximum Content Light Level. Static HDR metadata that corresponds to the brightest pixel in the entire stream. Measured in nits.</p>
    #[serde(rename = "MaxCll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cll: Option<i64>,
    /// <p>Maximum Frame-Average Light Level. Static HDR metadata that corresponds to the highest frame-average brightness in the entire stream. Measured in nits.</p>
    #[serde(rename = "MaxFall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fall: Option<i64>,
}

/// <p>Inserts DVB Network Information Table (NIT) at the specified table repetition interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbNitSettings {
    /// <p>The numeric value placed in the Network Information Table (NIT).</p>
    #[serde(rename = "NetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<i64>,
    /// <p>The network name text placed in the network<em>name</em>descriptor inside the Network Information Table. Maximum length is 256 characters.</p>
    #[serde(rename = "NetworkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "NitInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nit_interval: Option<i64>,
}

/// <p>Inserts DVB Service Description Table (NIT) at the specified table repetition interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DvbSdtSettings {
    /// <p>Selects method of inserting SDT information into output stream.  &quot;Follow input SDT&quot; copies SDT information from input stream to  output stream. &quot;Follow input SDT if present&quot; copies SDT information from  input stream to output stream if SDT information is present in the input, otherwise it will fall back on the user-defined values. Enter &quot;SDT  Manually&quot; means user will enter the SDT information. &quot;No SDT&quot; means output  stream will not contain SDT information.</p>
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
    /// <p>If no explicit x<em>position or y</em>position is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "Alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "BackgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>Provide the font script, using an ISO 15924 script code, if the LanguageCode is not sufficient for determining the script type. Where LanguageCode or CustomLanguageCode is sufficient, use &quot;AUTOMATIC&quot; or leave unset. This is used to help determine the appropriate font for rendering DVB-Sub captions.</p>
    #[serde(rename = "FontScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_script: Option<String>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "FontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "OutlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
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
    /// <p>Specify whether your DVB subtitles are standard or for hearing impaired. Choose hearing impaired if your subtitles include audio descriptions and dialogue. Choose standard if your subtitles include only dialogue.</p>
    #[serde(rename = "SubtitlingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitling_type: Option<String>,
    /// <p>Only applies to jobs with input captions in Teletext or STL formats. Specify whether the spacing between letters in your captions is set by the captions grid or varies depending on letter width. Choose fixed grid to conform to the spacing specified in the captions file more accurately. Choose proportional to make the text easier to read if the captions are closed caption.</p>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdt_interval: Option<i64>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3_ATMOS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eac3AtmosSettings {
    /// <p>Specify the average bitrate in bits per second.
    /// Valid values: 384k, 448k, 640k, 768k</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the E-AC-3 stream that the encoder emits. For more information about the EAC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>The coding mode for Dolby Digital Plus JOC (Atmos) is always 9.1.6 (CODING<em>MODE</em>9<em>1</em>6).</p>
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Enable Dolby Dialogue Intelligence to adjust loudness based on dialogue analysis.</p>
    #[serde(rename = "DialogueIntelligence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialogue_intelligence: Option<String>,
    /// <p>Specify the absolute peak level for a signal with dynamic range compression.</p>
    #[serde(rename = "DynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    /// <p>Specify how the service limits the audio dynamic range when compressing the audio.</p>
    #[serde(rename = "DynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left only/Right only center mix
    /// (Lo/Ro center). MediaConvert uses this value for downmixing. How the service uses this
    /// value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix).
    /// Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, and -6.0.</p>
    #[serde(rename = "LoRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left only/Right only (Lo/Ro surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel.</p>
    #[serde(rename = "LoRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left total/Right total center mix (Lt/Rt center). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, and -6.0.</p>
    #[serde(rename = "LtRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left total/Right total surround mix (Lt/Rt surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel.</p>
    #[serde(rename = "LtRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    /// <p>Choose how the service meters the loudness of your audio.</p>
    #[serde(rename = "MeteringMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_mode: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Specify the percentage of audio content that must be speech before the encoder uses the measured speech loudness as the overall program loudness.</p>
    #[serde(rename = "SpeechThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_threshold: Option<i64>,
    /// <p>Choose how the service does stereo downmixing.</p>
    #[serde(rename = "StereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    /// <p>Specify whether your input audio has an additional center rear surround channel matrix encoded into your left and right surround channels.</p>
    #[serde(rename = "SurroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eac3Settings {
    /// <p>If set to ATTENUATE<em>3</em>DB, applies a 3 dB attenuation to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "AttenuationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the E-AC-3 stream that the encoder emits. For more information about the EAC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "BitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital Plus coding mode. Determines number of channels.</p>
    #[serde(rename = "CodingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Activates a DC highpass filter for all input channels.</p>
    #[serde(rename = "DcFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital Plus, dialnorm will be passed through.</p>
    #[serde(rename = "Dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>Specify the absolute peak level for a signal with dynamic range compression.</p>
    #[serde(rename = "DynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    /// <p>Specify how the service limits the audio dynamic range when compressing the audio.</p>
    #[serde(rename = "DynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    /// <p>When encoding 3/2 audio, controls whether the LFE channel is enabled</p>
    #[serde(rename = "LfeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<String>,
    /// <p>Applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid with 3<em>2</em>LFE coding mode.</p>
    #[serde(rename = "LfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left only/Right only center mix (Lo/Ro center). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left only/Right only center (loRoCenterMixLevel).</p>
    #[serde(rename = "LoRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left only/Right only (Lo/Ro surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left only/Right only surround (loRoSurroundMixLevel).</p>
    #[serde(rename = "LoRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left total/Right total center mix (Lt/Rt center). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left total/Right total center (ltRtCenterMixLevel).</p>
    #[serde(rename = "LtRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left total/Right total surround mix (Lt/Rt surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left total/Right total surround (ltRtSurroundMixLevel).</p>
    #[serde(rename = "LtRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    /// <p>When set to FOLLOW_INPUT, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "MetadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>When set to WHEN_POSSIBLE, input DD+ audio will be passed through if it is present on the input. this detection is dynamic over the life of the transcode. Inputs that alternate between DD+ and non-DD+ content will have a consistent DD+ output as the system alternates between passthrough and encoding.</p>
    #[serde(rename = "PassthroughControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<String>,
    /// <p>Controls the amount of phase-shift applied to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "PhaseControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Choose how the service does stereo downmixing. This setting only applies if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Stereo downmix (Eac3StereoDownmix).</p>
    #[serde(rename = "StereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    /// <p>When encoding 3/2 audio, sets whether an extra center back surround channel is matrix encoded into the left and right surround channels.</p>
    #[serde(rename = "SurroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
    /// <p>When encoding 2/0 audio, sets whether Dolby Surround is matrix encoded into the two channels.</p>
    #[serde(rename = "SurroundMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_mode: Option<String>,
}

/// <p>Settings specific to embedded/ancillary caption outputs, including 608/708 Channel destination number.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedDestinationSettings {
    /// <p>Ignore this setting unless your input captions are SCC format and your output captions are embedded in the video stream. Specify a CC number for each captions channel in this output. If you have two channels, choose CC numbers that aren&#39;t in the same field. For example, choose 1 and 3. For more information, see https://docs.aws.amazon.com/console/mediaconvert/dual-scc-to-embedded.</p>
    #[serde(rename = "Destination608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_608_channel_number: Option<i64>,
    /// <p>Ignore this setting unless your input captions are SCC format and you want both 608 and 708 captions embedded in your output stream. Optionally, specify the 708 service number for each output captions channel. Choose a different number for each channel. To use this setting, also set Force 608 to 708 upconvert (Convert608To708) to Upconvert (UPCONVERT) in your input captions selector settings. If you choose to upconvert but don&#39;t specify a 708 service number, MediaConvert uses the number that you specify for CC channel number (destination608ChannelNumber) for the 708 service number. For more information, see https://docs.aws.amazon.com/console/mediaconvert/dual-scc-to-embedded.</p>
    #[serde(rename = "Destination708ServiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_708_service_number: Option<i64>,
}

/// <p>Settings for embedded captions Source</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedSourceSettings {
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
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
    /// <p>By default, the service terminates any unterminated captions at the end of each input. If you want the caption to continue onto your next input, disable this setting.</p>
    #[serde(rename = "TerminateCaptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_captions: Option<String>,
}

/// <p>Describes an account-specific API endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>URL of endpoint</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>ESAM ManifestConfirmConditionNotification defined by OC-SP-ESAM-API-I03-131025.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsamManifestConfirmConditionNotification {
    /// <p>Provide your ESAM ManifestConfirmConditionNotification XML document inside your JSON job settings. Form the XML document as per OC-SP-ESAM-API-I03-131025. The transcoder will use the Manifest Conditioning instructions in the message that you supply.</p>
    #[serde(rename = "MccXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc_xml: Option<String>,
}

/// <p>Settings for Event Signaling And Messaging (ESAM). If you don&#39;t do ad insertion, you can ignore these settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsamSettings {
    /// <p>Specifies an ESAM ManifestConfirmConditionNotification XML as per OC-SP-ESAM-API-I03-131025. The transcoder uses the manifest conditioning instructions that you provide in the setting MCC XML (mccXml).</p>
    #[serde(rename = "ManifestConfirmConditionNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_confirm_condition_notification: Option<EsamManifestConfirmConditionNotification>,
    /// <p>Specifies the stream distance, in milliseconds, between the SCTE 35 messages that the transcoder places and the splice points that they refer to. If the time between the start of the asset and the SCTE-35 message is less than this value, then the transcoder places the SCTE-35 marker at the beginning of the stream.</p>
    #[serde(rename = "ResponseSignalPreroll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_signal_preroll: Option<i64>,
    /// <p>Specifies an ESAM SignalProcessingNotification XML as per OC-SP-ESAM-API-I03-131025. The transcoder uses the signal processing instructions that you provide in the setting SCC XML (sccXml).</p>
    #[serde(rename = "SignalProcessingNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_processing_notification: Option<EsamSignalProcessingNotification>,
}

/// <p>ESAM SignalProcessingNotification data defined by OC-SP-ESAM-API-I03-131025.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsamSignalProcessingNotification {
    /// <p>Provide your ESAM SignalProcessingNotification XML document inside your JSON job settings. Form the XML document as per OC-SP-ESAM-API-I03-131025. The transcoder will use the signal processing instructions in the message that you supply. Provide your ESAM SignalProcessingNotification XML document inside your JSON job settings. For your MPEG2-TS file outputs, if you want the service to place SCTE-35 markers at the insertion points you specify in the XML document, you must also enable SCTE-35 ESAM (scte35Esam). Note that you can either specify an ESAM XML document or enable SCTE-35 passthrough. You can&#39;t do both.</p>
    #[serde(rename = "SccXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_xml: Option<String>,
}

/// <p>Settings for F4v container</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct F4vSettings {
    /// <p>If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.</p>
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
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
}

/// <p>If your input captions are SCC, SMI, SRT, STL, TTML, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSourceSettings {
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
    #[serde(rename = "Convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>External caption file used for loading captions. Accepted file extensions are &#39;scc&#39;, &#39;ttml&#39;, &#39;dfxp&#39;, &#39;stl&#39;, &#39;srt&#39;, &#39;xml&#39;, and &#39;smi&#39;.</p>
    #[serde(rename = "SourceFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file: Option<String>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobRequest {
    /// <p>the job ID of the job.</p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResponse {
    /// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "Job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobTemplateRequest {
    /// <p>The name of the job template.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPresetRequest {
    /// <p>The name of the preset.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueueRequest {
    /// <p>The name of the queue that you want information about.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Settings for quality-defined variable bitrate encoding with the H.264 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H264QvbrSettings {
    /// <p>Use this setting only when Rate control mode is QVBR and Quality tuning level is Multi-pass HQ. For Max average bitrate values suited to the complexity of your input video, the service limits the average bitrate of the video part of this output to the value that you choose. That is, the total size of the video element is less than or equal to the value you set multiplied by the number of seconds of encoded output.</p>
    #[serde(rename = "MaxAverageBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_average_bitrate: Option<i64>,
    /// <p>Required when you use QVBR rate control mode. That is, when you specify qvbrSettings within h264Settings. Specify the target quality level for this output, from 1 to 10. Use higher numbers for greater quality. Level 10 results in nearly lossless compression. The quality level for most broadcast-quality transcodes is between 6 and 9.</p>
    #[serde(rename = "QvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value H_264.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H264Settings {
    /// <p>Adaptive quantization. Allows intra-frame quantizers to vary to improve visual quality.</p>
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify an H.264 level that is consistent with your output video settings. If you aren&#39;t sure what level to specify, choose Auto (AUTO).</p>
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>H.264 Profile. High 4:2:2 and 10-bit profiles are only available with the AVC-I License.</p>
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "DynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>Entropy encoding mode. Use CABAC (must be in Main or High profile) or CAVLC.</p>
    #[serde(rename = "EntropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    /// <p>Choosing FORCE_FIELD disables PAFF encoding for interlaced outputs.</p>
    #[serde(rename = "FieldEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_encoding: Option<String>,
    /// <p>Adjust quantization within each frame to reduce flicker or &#39;pop&#39; on I-frames.</p>
    #[serde(rename = "FlickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>When set to INTERPOLATE, produces smoother motion during frame rate conversion.</p>
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Frame rate numerator - frame rate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>If enable, use reference B frames for GOP structures that have B frames &gt; 1.</p>
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
    /// <p>Indicates if the GOP Size in H264 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
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
    /// <p>Use Interlace mode (InterlaceMode) to choose the scan line type for the output. * Top Field First (TOP<em>FIELD) and Bottom Field First (BOTTOM</em>FIELD) produce interlaced output with the entire output having the same field polarity (top or bottom first). * Follow, Default Top (FOLLOW<em>TOP</em>FIELD) and Follow, Default Bottom (FOLLOW<em>BOTTOM</em>FIELD) use the same field polarity as the source. Therefore, behavior depends on the input scan type, as follows.
    /// - If the source is interlaced, the output will be interlaced with the same polarity as the source (it will follow the source). The output could therefore be a mix of &quot;top field first&quot; and &quot;bottom field first&quot;.
    /// - If the source is progressive, the output will be interlaced with &quot;top field first&quot; or &quot;bottom field first&quot; polarity, depending on which of the Follow options you chose.</p>
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. Required when Rate control mode is QVBR.</p>
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
    /// <p>Using the API, enable ParFollowSource if you want the service to use the pixel aspect ratio from the input. Using the console, do this by choosing Follow source for Pixel aspect ratio.</p>
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
    /// <p>Use Quality tuning level (H264QualityTuningLevel) to specifiy whether to use fast single-pass, high-quality singlepass, or high-quality multipass video encoding.</p>
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Settings for quality-defined variable bitrate encoding with the H.264 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
    #[serde(rename = "QvbrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_settings: Option<H264QvbrSettings>,
    /// <p>Use this setting to specify whether this output has a variable bitrate (VBR), constant bitrate (CBR) or quality-defined variable bitrate (QVBR).</p>
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Places a PPS header on each encoded picture, even if repeated.</p>
    #[serde(rename = "RepeatPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_pps: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default. If this output uses QVBR, choose Transition detection (TRANSITION_DETECTION) for further video quality improvement. For more information about QVBR, see https://docs.aws.amazon.com/console/mediaconvert/cbr-vbr-qvbr.</p>
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Enables Slow PAL rate conversion. 23.976fps and 24fps input is relabeled as 25fps, and audio is sped up correspondingly.</p>
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Softness. Selects quantizer matrix, larger values reduce high-frequency content in the encoded image.</p>
    #[serde(rename = "Softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>Adjust quantization within each frame based on spatial variation of content complexity.</p>
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>Produces a bitstream compliant with SMPTE RP-2027.</p>
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// <p>This field applies only if the Streams &gt; Advanced &gt; Framerate (framerate) field  is set to 29.970. This field works with the Streams &gt; Advanced &gt; Preprocessors &gt; Deinterlacer  field (deinterlace<em>mode) and the Streams &gt; Advanced &gt; Interlaced Mode field (interlace</em>mode)  to identify the scan type for the output: Progressive, Interlaced, Hard Telecine or Soft Telecine. - Hard: produces 29.97i output from 23.976 input. - Soft: produces 23.976; the player converts this output to 29.97i.</p>
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Adjust quantization within each frame based on temporal variation of content complexity.</p>
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    /// <p>Inserts timecode for each frame as 4 bytes of an unregistered SEI message.</p>
    #[serde(rename = "UnregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
}

/// <p>Settings for quality-defined variable bitrate encoding with the H.265 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H265QvbrSettings {
    /// <p>Use this setting only when Rate control mode is QVBR and Quality tuning level is Multi-pass HQ. For Max average bitrate values suited to the complexity of your input video, the service limits the average bitrate of the video part of this output to the value that you choose. That is, the total size of the video element is less than or equal to the value you set multiplied by the number of seconds of encoded output.</p>
    #[serde(rename = "MaxAverageBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_average_bitrate: Option<i64>,
    /// <p>Required when you use QVBR rate control mode. That is, when you specify qvbrSettings within h265Settings. Specify the target quality level for this output, from 1 to 10. Use higher numbers for greater quality. Level 10 results in nearly lossless compression. The quality level for most broadcast-quality transcodes is between 6 and 9.</p>
    #[serde(rename = "QvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
}

/// <p>Settings for H265 codec</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H265Settings {
    /// <p>Adaptive quantization. Allows intra-frame quantizers to vary to improve visual quality.</p>
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Enables Alternate Transfer Function SEI message for outputs using Hybrid Log Gamma (HLG) Electro-Optical Transfer Function (EOTF).</p>
    #[serde(rename = "AlternateTransferFunctionSei")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_transfer_function_sei: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>H.265 Level.</p>
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>Represents the Profile and Tier, per the HEVC (H.265) specification. Selections are grouped as [Profile] / [Tier], so &quot;Main/High&quot; represents Main Profile with High Tier. 4:2:2 profiles are only available with the HEVC 4:2:2 License.</p>
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "DynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>Adjust quantization within each frame to reduce flicker or &#39;pop&#39; on I-frames.</p>
    #[serde(rename = "FlickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job sepecification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>When set to INTERPOLATE, produces smoother motion during frame rate conversion.</p>
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Frame rate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Frame rate numerator - frame rate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>If enable, use reference B frames for GOP structures that have B frames &gt; 1.</p>
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
    /// <p>Indicates if the GOP Size in H265 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
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
    /// <p>Choose the scan line type for the output. Choose Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Choose Top Field First (TOP<em>FIELD) or Bottom Field First (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Choose Follow, Default Top (FOLLOW<em>TOP</em>FIELD) or Follow, Default Bottom (FOLLOW<em>BOTTOM</em>FIELD) to create an interlaced output with the same field polarity as the source. If the source is interlaced, the output will be interlaced with the same polarity as the source (it will follow the source). The output could therefore be a mix of &quot;top field first&quot; and &quot;bottom field first&quot;. If the source is progressive, your output will be interlaced with &quot;top field first&quot; or &quot;bottom field first&quot; polarity, depending on which of the Follow options you chose. If you don&#39;t choose a value, the service will default to Progressive (PROGRESSIVE).</p>
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. Required when Rate control mode is QVBR.</p>
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
    /// <p>Using the API, enable ParFollowSource if you want the service to use the pixel aspect ratio from the input. Using the console, do this by choosing Follow source for Pixel aspect ratio.</p>
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
    /// <p>Use Quality tuning level (H265QualityTuningLevel) to specifiy whether to use fast single-pass, high-quality singlepass, or high-quality multipass video encoding.</p>
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Settings for quality-defined variable bitrate encoding with the H.265 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
    #[serde(rename = "QvbrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_settings: Option<H265QvbrSettings>,
    /// <p>Use this setting to specify whether this output has a variable bitrate (VBR), constant bitrate (CBR) or quality-defined variable bitrate (QVBR).</p>
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Specify Sample Adaptive Offset (SAO) filter strength.  Adaptive mode dynamically selects best strength based on content</p>
    #[serde(rename = "SampleAdaptiveOffsetFilterMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_adaptive_offset_filter_mode: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default. If this output uses QVBR, choose Transition detection (TRANSITION_DETECTION) for further video quality improvement. For more information about QVBR, see https://docs.aws.amazon.com/console/mediaconvert/cbr-vbr-qvbr.</p>
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "Slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Enables Slow PAL rate conversion. 23.976fps and 24fps input is relabeled as 25fps, and audio is sped up correspondingly.</p>
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Adjust quantization within each frame based on spatial variation of content complexity.</p>
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>This field applies only if the Streams &gt; Advanced &gt; Framerate (framerate) field  is set to 29.970. This field works with the Streams &gt; Advanced &gt; Preprocessors &gt; Deinterlacer  field (deinterlace<em>mode) and the Streams &gt; Advanced &gt; Interlaced Mode field (interlace</em>mode)  to identify the scan type for the output: Progressive, Interlaced, Hard Telecine or Soft Telecine. - Hard: produces 29.97i output from 23.976 input. - Soft: produces 23.976; the player converts this output to 29.97i.</p>
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Adjust quantization within each frame based on temporal variation of content complexity.</p>
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    /// <p>Enables temporal layer identifiers in the encoded bitstream. Up to 3 layers are supported depending on GOP structure: I- and P-frames form one layer, reference B-frames can form a second layer and non-reference b-frames can form a third layer. Decoders can optionally decode only the lower temporal layers to generate a lower frame rate output. For example, given a bitstream with temporal IDs and with b-frames = 1 (i.e. IbPbPb display order), a decoder could decode all the frames for full frame rate output or only the I and P frames (lowest temporal layer) for a half frame rate output.</p>
    #[serde(rename = "TemporalIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_ids: Option<String>,
    /// <p>Enable use of tiles, allowing horizontal as well as vertical subdivision of the encoded pictures.</p>
    #[serde(rename = "Tiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<String>,
    /// <p>Inserts timecode for each frame as 4 bytes of an unregistered SEI message.</p>
    #[serde(rename = "UnregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
    /// <p>If the location of parameter set NAL units doesn&#39;t matter in your workflow, ignore this setting. Use this setting only with CMAF or DASH outputs, or with standalone file outputs in an MPEG-4 container (MP4 outputs). Choose HVC1 to mark your output as HVC1. This makes your output compliant with the following specification: ISO IECJTC1 SC29 N13798 Text ISO/IEC FDIS 14496-15 3rd Edition. For these outputs, the service stores parameter set NAL units in the sample headers but not in the samples directly. For MP4 outputs, when you choose HVC1, your output video might not work properly with some downstream systems and video players. The service defaults to marking your output as HEV1. For these outputs, the service writes parameter set NAL units directly into the samples.</p>
    #[serde(rename = "WriteMp4PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_mp_4_packaging_type: Option<String>,
}

/// <p>Use these settings to specify static color calibration metadata, as defined by SMPTE ST 2086. These values don&#39;t affect the pixel values that are encoded in the video stream. They are intended to help the downstream video player display content in a way that reflects the intentions of the the content creator.</p>
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
    /// <p>Maximum light level among all samples in the coded video sequence, in units of candelas per square meter.  This setting doesn&#39;t have a default value; you must specify a value that is suitable for the content.</p>
    #[serde(rename = "MaxContentLightLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_content_light_level: Option<i64>,
    /// <p>Maximum average light level of any frame in the coded video sequence, in units of candelas per square meter. This setting doesn&#39;t have a default value; you must specify a value that is suitable for the content.</p>
    #[serde(rename = "MaxFrameAverageLightLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_average_light_level: Option<i64>,
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

/// <p>Specify the details for each additional HLS manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your HLS group is film-name.m3u8. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.m3u8. For HLS output groups, specify a manifestNameModifier that is different from the nameModifier of the output. The service uses the output name modifier to create unique names for the individual variant manifests.</p>
    #[serde(rename = "ManifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "SelectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Caption Language Mapping</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsCaptionLanguageMapping {
    /// <p>Caption channel.</p>
    #[serde(rename = "CaptionChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_channel: Option<i64>,
    /// <p>Specify the language for this captions channel, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "CustomLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Specify the language, using the ISO 639-2 three-letter code listed at https://www.loc.gov/standards/iso639-2/php/code_list.php.</p>
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
    /// <p>Encrypts the segments with the given encryption scheme. Leave blank to disable. Selecting &#39;Disabled&#39; in the web interface also disables encryption.</p>
    #[serde(rename = "EncryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>The Initialization Vector is a 128-bit number used in conjunction with the key for encrypting blocks. If set to INCLUDE, Initialization Vector is listed in the manifest. Otherwise Initialization Vector is not in the manifest.</p>
    #[serde(rename = "InitializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    /// <p>Enable this setting to insert the EXT-X-SESSION-KEY element into the master playlist. This allows for offline Apple HLS FairPlay content protection.</p>
    #[serde(rename = "OfflineEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_encrypted: Option<String>,
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
    /// <p>Use these settings to set up encryption with a static key provider.</p>
    #[serde(rename = "StaticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    /// <p>Specify whether your DRM encryption key is static or from a key provider that follows the SPEKE standard. For more information about SPEKE, see https://docs.aws.amazon.com/speke/latest/documentation/what-is-speke.html.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to HLS<em>GROUP</em>SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsGroupSettings {
    /// <p>Choose one or more ad marker types to decorate your Apple HLS manifest. This setting does not determine whether SCTE-35 markers appear in the outputs themselves.</p>
    #[serde(rename = "AdMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    /// <p>By default, the service creates one top-level .m3u8 HLS manifest for each HLS output group in your job. This default manifest references every output in the output group. To create additional top-level manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "AdditionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<HlsAdditionalManifest>>,
    /// <p>A partial URI prefix that will be prepended to each output in the media .m3u8 file. Can be used if base manifest is delivered from a different URL than the main .m3u8 file.</p>
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Language to be used on Caption outputs</p>
    #[serde(rename = "CaptionLanguageMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<HlsCaptionLanguageMapping>>,
    /// <p>Applies only to 608 Embedded output captions. Insert: Include CLOSED-CAPTIONS lines in the manifest. Specify at least one language in the CC1 Language Code field. One CLOSED-CAPTION line is added for each Language Code you specify. Make sure to specify the languages in the order in which they appear in the original source (if the source is embedded format) or the order of the caption selectors (if the source is other than embedded). Otherwise, languages in the manifest will not match up properly with the output captions. None: Include CLOSED-CAPTIONS=NONE line in the manifest. Omit: Omit any CLOSED-CAPTIONS line from the manifest.</p>
    #[serde(rename = "CaptionLanguageSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<String>,
    /// <p>When set to ENABLED, sets #EXT-X-ALLOW-CACHE:no tag, which prevents client from saving media segments for later replay.</p>
    #[serde(rename = "ClientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    /// <p>Specification to use (RFC-6381 or the default RFC-4281) during m3u8 playlist generation.</p>
    #[serde(rename = "CodecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>Indicates whether segments should be placed in subdirectories.</p>
    #[serde(rename = "DirectoryStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<String>,
    /// <p>DRM settings.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryptionSettings>,
    /// <p>When set to GZIP, compresses HLS playlist.</p>
    #[serde(rename = "ManifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    /// <p>Indicates whether the output manifest should use floating point values for segment duration.</p>
    #[serde(rename = "ManifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>Keep this setting at the default value of 0, unless you are troubleshooting a problem with how devices play back the end of your video asset. If you know that player devices are hanging on the final segment of your video because the length of your final segment is too short, use this setting to specify a minimum final segment length, in seconds. Choose a value that is greater than or equal to 1 and less than your segment length. When you specify a value for this setting, the encoder will combine any final segment that is shorter than the length that you specify with the previous segment. For example, your segment length is 3 seconds and your final segment is .5 seconds without a minimum final segment length; when you set the minimum final segment length to 1, your final segment is 3.5 seconds.</p>
    #[serde(rename = "MinFinalSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_final_segment_length: Option<f64>,
    /// <p>When set, Minimum Segment Size is enforced by looking ahead and back within the specified range for a nearby avail and extending the segment size if needed.</p>
    #[serde(rename = "MinSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_length: Option<i64>,
    /// <p>Indicates whether the .m3u8 manifest file should be generated for this HLS output group.</p>
    #[serde(rename = "OutputSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<String>,
    /// <p>Includes or excludes EXT-X-PROGRAM-DATE-TIME tag in .m3u8 manifest files. The value is calculated as follows: either the program date and time are initialized using the input timecode source, or the time is initialized using the input timecode source and the date is initialized using the timestamp_offset.</p>
    #[serde(rename = "ProgramDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<String>,
    /// <p>Period of insertion of EXT-X-PROGRAM-DATE-TIME entry, in seconds.</p>
    #[serde(rename = "ProgramDateTimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i64>,
    /// <p>When set to SINGLE_FILE, emits program as a single media resource (.ts) file, uses #EXT-X-BYTERANGE tags to index segment for playback.</p>
    #[serde(rename = "SegmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of MPEG-2 Transport Stream segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer.</p>
    #[serde(rename = "SegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>Number of segments to write to a subdirectory before starting a new one. directoryStructure must be SINGLE_DIRECTORY for this setting to have an effect.</p>
    #[serde(rename = "SegmentsPerSubdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_per_subdirectory: Option<i64>,
    /// <p>Include or exclude RESOLUTION attribute for video in EXT-X-STREAM-INF tag of variant manifest.</p>
    #[serde(rename = "StreamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    /// <p>Indicates ID3 frame that has the timecode.</p>
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
    /// <p>Use this setting only in audio-only outputs. Choose MPEG-2 Transport Stream (M2TS) to create a file in an MPEG2-TS container. Keep the default value Automatic (AUTOMATIC) to create an audio-only file in a raw container. Regardless of the value that you specify here, if this output has video, the service will place the output into an MPEG2-TS container.</p>
    #[serde(rename = "AudioOnlyContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_container: Option<String>,
    /// <p>List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by &#39;,&#39;.</p>
    #[serde(rename = "AudioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    /// <p>Four types of audio-only tracks are supported: Audio-Only Variant Stream The client can play back this audio-only stream instead of video in low-bandwidth scenarios. Represented as an EXT-X-STREAM-INF in the HLS manifest. Alternate Audio, Auto Select, Default Alternate rendition that the client should try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=YES, AUTOSELECT=YES Alternate Audio, Auto Select, Not Default Alternate rendition that the client may try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=YES Alternate Audio, not Auto Select Alternate rendition that the client will not try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=NO</p>
    #[serde(rename = "AudioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    /// <p>When set to INCLUDE, writes I-Frame Only Manifest in addition to the HLS manifest</p>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_3: Option<String>,
    /// <p>Provide a Timecode (TimeCode) in HH:MM:SS:FF or HH:MM:SS;FF format.</p>
    #[serde(rename = "Timecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

/// <p>Enable the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input or output individually. This setting is disabled by default.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInserter {
    /// <p>Specify the images that you want to overlay on your video. The images must be PNG or TGA files.</p>
    #[serde(rename = "InsertableImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertable_images: Option<Vec<InsertableImage>>,
}

/// <p>Settings specific to IMSC caption outputs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImscDestinationSettings {
    /// <p>Keep this setting enabled to have MediaConvert use the font style and position information from the captions source in the output. This option is available only when your input captions are CFF-TT, IMSC, SMPTE-TT, or TTML. Disable this setting for simplified output captions.</p>
    #[serde(rename = "StylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
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
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame. If you specify a value here, it will override any value that you specify in the output setting Cropping selection (crop).</p>
    #[serde(rename = "Crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Enable Deblock (InputDeblockFilter) to produce smoother motion in the output. Default is disabled. Only manaully controllable for MPEG2 and uncompressed video inputs.</p>
    #[serde(rename = "DeblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    /// <p>Settings for decrypting any input files that you encrypt before you upload them to Amazon S3. MediaConvert can decrypt files only when you use AWS Key Management Service (KMS) to encrypt the data key that you use to encrypt your content.</p>
    #[serde(rename = "DecryptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_settings: Option<InputDecryptionSettings>,
    /// <p>Enable Denoise (InputDenoiseFilter) to filter noise from the input.  Default is disabled. Only applicable to MPEG2, H.264, H.265, and uncompressed video inputs.</p>
    #[serde(rename = "DenoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Specify the source file for your transcoding job. You can use multiple inputs in a single job. The service concatenates these inputs, in the order that you specify them in the job, to create the outputs. If your input format is IMF, specify your input by providing the path to your CPL. For example, &quot;s3://bucket/vf/cpl.xml&quot;. If the CPL is in an incomplete IMP, make sure to use <em>Supplemental IMPs</em> (SupplementalImps) to specify any supplemental IMPs that contain assets referenced by the CPL.</p>
    #[serde(rename = "FileInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_input: Option<String>,
    /// <p>Use Filter enable (InputFilterEnable) to specify how the transcoding service applies the denoise and deblock filters. You must also enable the filters separately, with Denoise (InputDenoiseFilter) and Deblock (InputDeblockFilter). * Auto - The transcoding service determines whether to apply filtering, depending on input type and quality. * Disable - The input is not filtered. This is true even if you use the API to enable them in (InputDeblockFilter) and (InputDeblockFilter). * Force - The in put is filtered regardless of input type.</p>
    #[serde(rename = "FilterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "FilterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>Enable the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input individually. This setting is disabled by default.</p>
    #[serde(rename = "ImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "InputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black. If you specify a value here, it will override any value that you specify in the output setting Selection placement (position). If you specify a value here, this will override any AFD values in your input, even if you set Respond to AFD (RespondToAfd) to Respond (RESPOND). If you specify a value here, this will ignore anything that you specify for the setting Scaling Behavior (scalingBehavior).</p>
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Set PSI control (InputPsiControl) for transport stream inputs to specify which data the demux process to scans. * Ignore PSI - Scan all PIDs for audio and video. * Use PSI - Scan only PSI data.</p>
    #[serde(rename = "PsiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    /// <p>Provide a list of any necessary supplemental IMPs. You need supplemental IMPs if the CPL that you&#39;re using for your input is in an incomplete IMP. Specify either the supplemental IMP directories with a trailing slash or the ASSETMAP.xml files. For example [&quot;s3://bucket/ov/&quot;, &quot;s3://bucket/vf2/ASSETMAP.xml&quot;]. You don&#39;t need to specify the IMP that contains your input CPL, because the service automatically detects it.</p>
    #[serde(rename = "SupplementalImps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_imps: Option<Vec<String>>,
    /// <p>Use this Timecode source setting, located under the input settings (InputTimecodeSource), to specify how the service counts input video frames. This input frame count affects only the behavior of features that apply to a single input at a time, such as input clipping and synchronizing some captions formats. Choose Embedded (EMBEDDED) to use the timecodes in your input video. Choose Start at zero (ZEROBASED) to start the first frame at zero. Choose Specified start (SPECIFIEDSTART) to start the first frame at the timecode that you specify in the setting Start timecode (timecodeStart). If you don&#39;t specify a value for Timecode source, the service will use Embedded by default. For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "TimecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    /// <p>Specify the timecode that you want the service to use for this input&#39;s initial frame. To use this setting, you must set the Timecode source setting, located under the input settings (InputTimecodeSource), to Specified start (SPECIFIEDSTART). For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "TimecodeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_start: Option<String>,
    /// <p>Selector for video.</p>
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

/// <p>Settings for decrypting any input files that you encrypt before you upload them to Amazon S3. MediaConvert can decrypt files only when you use AWS Key Management Service (KMS) to encrypt the data key that you use to encrypt your content.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDecryptionSettings {
    /// <p>Specify the encryption mode that you used to encrypt your input files.</p>
    #[serde(rename = "DecryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_mode: Option<String>,
    /// <p>Warning! Don&#39;t provide your encryption key in plaintext. Your job settings could be intercepted, making your encrypted content vulnerable. Specify the encrypted version of the data key that you used to encrypt your content. The data key must be encrypted by AWS Key Management Service (KMS). The key can be 128, 192, or 256 bits.</p>
    #[serde(rename = "EncryptedDecryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_decryption_key: Option<String>,
    /// <p>Specify the initialization vector that you used when you encrypted your content before uploading it to Amazon S3. You can use a 16-byte initialization vector with any encryption mode. Or, you can use a 12-byte initialization vector with GCM or CTR. MediaConvert accepts only initialization vectors that are base64-encoded.</p>
    #[serde(rename = "InitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector: Option<String>,
    /// <p>Specify the AWS Region for AWS Key Management Service (KMS) that you used to encrypt your data key, if that Region is different from the one you are using for AWS Elemental MediaConvert.</p>
    #[serde(rename = "KmsKeyRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_region: Option<String>,
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
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame. If you specify a value here, it will override any value that you specify in the output setting Cropping selection (crop).</p>
    #[serde(rename = "Crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Enable Deblock (InputDeblockFilter) to produce smoother motion in the output. Default is disabled. Only manaully controllable for MPEG2 and uncompressed video inputs.</p>
    #[serde(rename = "DeblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    /// <p>Enable Denoise (InputDenoiseFilter) to filter noise from the input.  Default is disabled. Only applicable to MPEG2, H.264, H.265, and uncompressed video inputs.</p>
    #[serde(rename = "DenoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Use Filter enable (InputFilterEnable) to specify how the transcoding service applies the denoise and deblock filters. You must also enable the filters separately, with Denoise (InputDenoiseFilter) and Deblock (InputDeblockFilter). * Auto - The transcoding service determines whether to apply filtering, depending on input type and quality. * Disable - The input is not filtered. This is true even if you use the API to enable them in (InputDeblockFilter) and (InputDeblockFilter). * Force - The in put is filtered regardless of input type.</p>
    #[serde(rename = "FilterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "FilterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>Enable the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input individually. This setting is disabled by default.</p>
    #[serde(rename = "ImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "InputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black. If you specify a value here, it will override any value that you specify in the output setting Selection placement (position). If you specify a value here, this will override any AFD values in your input, even if you set Respond to AFD (RespondToAfd) to Respond (RESPOND). If you specify a value here, this will ignore anything that you specify for the setting Scaling Behavior (scalingBehavior).</p>
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Set PSI control (InputPsiControl) for transport stream inputs to specify which data the demux process to scans. * Ignore PSI - Scan all PIDs for audio and video. * Use PSI - Scan only PSI data.</p>
    #[serde(rename = "PsiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    /// <p>Use this Timecode source setting, located under the input settings (InputTimecodeSource), to specify how the service counts input video frames. This input frame count affects only the behavior of features that apply to a single input at a time, such as input clipping and synchronizing some captions formats. Choose Embedded (EMBEDDED) to use the timecodes in your input video. Choose Start at zero (ZEROBASED) to start the first frame at zero. Choose Specified start (SPECIFIEDSTART) to start the first frame at the timecode that you specify in the setting Start timecode (timecodeStart). If you don&#39;t specify a value for Timecode source, the service will use Embedded by default. For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "TimecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    /// <p>Specify the timecode that you want the service to use for this input&#39;s initial frame. To use this setting, you must set the Timecode source setting, located under the input settings (InputTimecodeSource), to Specified start (SPECIFIEDSTART). For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "TimecodeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_start: Option<String>,
    /// <p>Selector for video.</p>
    #[serde(rename = "VideoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>Settings that specify how your still graphic overlay appears.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertableImage {
    /// <p>Specify the time, in milliseconds, for the image to remain on the output video. This duration includes fade-in time but not fade-out time.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Specify the length of time, in milliseconds, between the Start time that you specify for the image insertion and the time that the image appears at full opacity. Full opacity is the level that you specify for the opacity setting. If you don&#39;t specify a value for Fade-in, the image will appear abruptly at the overlay start time.</p>
    #[serde(rename = "FadeIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i64>,
    /// <p>Specify the length of time, in milliseconds, between the end of the time that you have specified for the image overlay Duration and when the overlaid image has faded to total transparency. If you don&#39;t specify a value for Fade-out, the image will disappear abruptly at the end of the inserted image duration.</p>
    #[serde(rename = "FadeOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i64>,
    /// <p>Specify the height of the inserted image in pixels. If you specify a value that&#39;s larger than the video resolution height, the service will crop your overlaid image to fit. To use the native height of the image, keep this setting blank.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Specify the HTTP, HTTPS, or Amazon S3 location of the image that you want to overlay on the video. Use a PNG or TGA file.</p>
    #[serde(rename = "ImageInserterInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter_input: Option<String>,
    /// <p>Specify the distance, in pixels, between the inserted image and the left edge of the video frame. Required for any image overlay that you specify.</p>
    #[serde(rename = "ImageX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i64>,
    /// <p>Specify the distance, in pixels, between the overlaid image and the top edge of the video frame. Required for any image overlay that you specify.</p>
    #[serde(rename = "ImageY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i64>,
    /// <p>Specify how overlapping inserted images appear. Images with higher values for Layer appear on top of images with lower values for Layer.</p>
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i64>,
    /// <p>Use Opacity (Opacity) to specify how much of the underlying video shows through the inserted image. 0 is transparent and 100 is fully opaque. Default is 50.</p>
    #[serde(rename = "Opacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i64>,
    /// <p>Specify the timecode of the frame that you want the overlay to first appear on. This must be in timecode (HH:MM:SS:FF or HH:MM:SS;FF) format. Remember to take into account your timecode source settings.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>Specify the width of the inserted image in pixels. If you specify a value that&#39;s larger than the video resolution width, the service will crop your overlaid image to fit. To use the native width of the image, keep this setting blank.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content.</p>
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>Describes whether the current job is running with accelerated transcoding. For jobs that have Acceleration (AccelerationMode) set to DISABLED, AccelerationStatus is always NOT<em>APPLICABLE. For jobs that have Acceleration (AccelerationMode) set to ENABLED or PREFERRED, AccelerationStatus is one of the other states. AccelerationStatus is IN</em>PROGRESS initially, while the service determines whether the input files and job settings are compatible with accelerated transcoding. If they are, AcclerationStatus is ACCELERATED. If your input files and job settings aren&#39;t compatible with accelerated transcoding, the service either fails your job or runs it without accelerated transcoding, depending on how you set Acceleration (AccelerationMode). When the service runs your job without accelerated transcoding, AccelerationStatus is NOT_ACCELERATED.</p>
    #[serde(rename = "AccelerationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_status: Option<String>,
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don&#39;t have an associated tag will appear in your billing report unsorted. If you don&#39;t choose a valid value for this field, your job outputs will appear on the billing report unsorted.</p>
    #[serde(rename = "BillingTagsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tags_source: Option<String>,
    /// <p>The time, in Unix epoch format in seconds, when the job got created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A job&#39;s phase can be PROBING, TRANSCODING OR UPLOADING</p>
    #[serde(rename = "CurrentPhase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
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
    /// <p>An estimate of how far your job has progressed. This estimate is shown as a percentage of the total time from when your job leaves its queue to when your output files appear in your output Amazon S3 bucket. AWS Elemental MediaConvert provides jobPercentComplete in CloudWatch STATUS_UPDATE events and in the response to GetJob and ListJobs requests. The jobPercentComplete estimate is reliable for the following input containers: Quicktime, Transport Stream, MP4, and MXF. For some jobs, the service can&#39;t provide information about job progress. In those cases, jobPercentComplete returns a null value.</p>
    #[serde(rename = "JobPercentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_percent_complete: Option<i64>,
    /// <p>The job template that the job is created from, if it is created from a job template.</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>Provides messages from the service about jobs that you have already successfully submitted.</p>
    #[serde(rename = "Messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<JobMessages>,
    /// <p>List of output group details</p>
    #[serde(rename = "OutputGroupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_details: Option<Vec<OutputGroupDetail>>,
    /// <p>Relative priority on the job.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>The number of times that the service automatically attempted to process your job after encountering an error.</p>
    #[serde(rename = "RetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    /// <p>The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>JobSettings contains all the transcode settings for a job.</p>
    #[serde(rename = "Settings")]
    pub settings: JobSettings,
    /// <p>Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.</p>
    #[serde(rename = "SimulateReservedQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulate_reserved_queue: Option<String>,
    /// <p>A job&#39;s status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>Information about when jobs are submitted, started, and finished is specified in Unix epoch format in seconds.</p>
    #[serde(rename = "Timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    /// <p>User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.</p>
    #[serde(rename = "UserMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides messages from the service about jobs that you have already successfully submitted.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobMessages {
    /// <p>List of messages that are informational only and don&#39;t indicate a problem with your job.</p>
    #[serde(rename = "Info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Vec<String>>,
    /// <p>List of messages that warn about conditions that might cause your job not to run or to fail.</p>
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<Vec<String>>,
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
    /// <p>Settings for Event Signaling And Messaging (ESAM).</p>
    #[serde(rename = "Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<EsamSettings>,
    /// <p>Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    /// <p>Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups.</p>
    #[serde(rename = "MotionImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_image_inserter: Option<MotionImageInserter>,
    /// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
    #[serde(rename = "NielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "OutputGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_groups: Option<Vec<OutputGroup>>,
    /// <p>Contains settings used to acquire and adjust timecode information from inputs.</p>
    #[serde(rename = "TimecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    /// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in your job. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
    #[serde(rename = "TimedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

/// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobTemplate {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content.</p>
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
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
    /// <p>Relative priority on the job.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "Settings")]
    pub settings: JobTemplateSettings,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
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
    /// <p>Settings for Event Signaling And Messaging (ESAM).</p>
    #[serde(rename = "Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<EsamSettings>,
    /// <p>Use Inputs (inputs) to define the source file used in the transcode job. There can only be one input in a job template.  Using the API, you can include multiple inputs when referencing a job template.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputTemplate>>,
    /// <p>Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups.</p>
    #[serde(rename = "MotionImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_image_inserter: Option<MotionImageInserter>,
    /// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
    #[serde(rename = "NielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "OutputGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_groups: Option<Vec<OutputGroup>>,
    /// <p>Contains settings used to acquire and adjust timecode information from inputs.</p>
    #[serde(rename = "TimecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    /// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in your job. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
    #[serde(rename = "TimedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobTemplatesRequest {
    /// <p>Optionally, specify a job template category to limit responses to only job templates from that category.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. When you request a list of job templates, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by name.</p>
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
    /// <p>When you request lists of resources, you can optionally specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>Optional. Number of jobs, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When you request lists of resources, you can optionally specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>Provide a queue name to get back only jobs from that queue.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>A job&#39;s status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPresetsRequest {
    /// <p>Optionally, specify a preset category to limit responses to only presets from that category.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. When you request a list of presets, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by name.</p>
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
    /// <p>When you request lists of resources, you can optionally specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQueuesRequest {
    /// <p>Optional. When you request a list of queues, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by creation date.</p>
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
    /// <p>When you request lists of resources, you can optionally specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQueuesResponse {
    /// <p>Use this string to request the next batch of queues.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of queues.</p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<Queue>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to list tags for. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The Amazon Resource Name (ARN) and tags for an AWS Elemental MediaConvert resource.</p>
    #[serde(rename = "ResourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<ResourceTags>,
}

/// <p>Settings for SCTE-35 signals from ESAM. Include this in your job settings to put SCTE-35 markers in your HLS and transport stream outputs at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M2tsScte35Esam {
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.</p>
    #[serde(rename = "Scte35EsamPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam_pid: Option<i64>,
}

/// <p>MPEG-2 TS container settings. These apply to outputs in a File output group when the output&#39;s container (ContainerType) is MPEG-2 Transport Stream (M2TS). In these assets, data is organized by the program map table (PMT). Each transport stream program contains subsets of data, including audio, video, and metadata. Each of these subsets of data has a numerical label called a packet identifier (PID). Each transport stream program corresponds to one MediaConvert output. The PMT lists the types of data in a program along with their PID. Downstream systems and players use the program map table to look up the PID for each type of data it accesses and then uses the PIDs to locate specific data within the asset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M2tsSettings {
    /// <p>Selects between the DVB and ATSC buffer models for Dolby Digital audio.</p>
    #[serde(rename = "AudioBufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "AudioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Specify the packet identifiers (PIDs) for any elementary audio streams you include in this output. Specify multiple PIDs as a JSON array. Default is the range 482-492.</p>
    #[serde(rename = "AudioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    /// <p>Specify the output bitrate of the transport stream in bits per second. Setting to 0 lets the muxer automatically determine the appropriate bitrate. Other common values are 3750000, 7500000, and 15000000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Controls what buffer model to use for accurate interleaving. If set to MULTIPLEX, use multiplex  buffer model. If set to NONE, this can lead to lower latency, but low-memory devices may not be able to play back the stream without interruptions.</p>
    #[serde(rename = "BufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<String>,
    /// <p>Inserts DVB Network Information Table (NIT) at the specified table repetition interval.</p>
    #[serde(rename = "DvbNitSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    /// <p>Inserts DVB Service Description Table (NIT) at the specified table repetition interval.</p>
    #[serde(rename = "DvbSdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    /// <p>Specify the packet identifiers (PIDs) for DVB subtitle data included in this output. Specify multiple PIDs as a JSON array. Default is the range 460-479.</p>
    #[serde(rename = "DvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<Vec<i64>>,
    /// <p>Inserts DVB Time and Date Table (TDT) at the specified table repetition interval.</p>
    #[serde(rename = "DvbTdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    /// <p>Specify the packet identifier (PID) for DVB teletext data you include in this output. Default is 499.</p>
    #[serde(rename = "DvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<i64>,
    /// <p>When set to VIDEO<em>AND</em>FIXED<em>INTERVALS, audio EBP markers will be added to partitions 3 and 4. The interval between these additional markers will be fixed, and will be slightly shorter than the video EBP marker interval. When set to VIDEO</em>INTERVAL, these additional markers will not be inserted. Only applicable when EBP segmentation markers are is selected (segmentationMarkers is EBP or EBP_LEGACY).</p>
    #[serde(rename = "EbpAudioInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<String>,
    /// <p>Selects which PIDs to place EBP markers on. They can either be placed only on the video PID, or on both the video PID and all audio PIDs. Only applicable when EBP segmentation markers are is selected (segmentationMarkers is EBP or EBP_LEGACY).</p>
    #[serde(rename = "EbpPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<String>,
    /// <p>Controls whether to include the ES Rate field in the PES header.</p>
    #[serde(rename = "EsRateInPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    /// <p>Keep the default value (DEFAULT) unless you know that your audio EBP markers are incorrectly appearing before your video EBP markers. To correct this problem, set this value to Force (FORCE).</p>
    #[serde(rename = "ForceTsVideoEbpOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_ts_video_ebp_order: Option<String>,
    /// <p>The length, in seconds, of each fragment. Only used with EBP markers.</p>
    #[serde(rename = "FragmentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,
    /// <p>Specify the maximum time, in milliseconds, between Program Clock References (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "MaxPcrInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pcr_interval: Option<i64>,
    /// <p>When set, enforces that Encoder Boundary Points do not come within the specified time interval of each other by looking ahead at input video. If another EBP is going to come in within the specified time interval, the current EBP is not emitted, and the segment is &quot;stretched&quot; to the next marker. The lookahead value does not add latency to the system. The Live Event must be configured elsewhere to create sufficient latency to make the lookahead accurate.</p>
    #[serde(rename = "MinEbpInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ebp_interval: Option<i64>,
    /// <p>If INSERT, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
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
    /// <p>When set to PCR<em>EVERY</em>PES_PACKET, a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
    #[serde(rename = "PcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Specify the packet identifier (PID) for the program clock reference (PCR) in this output. If you do not specify a value, the service will use the value for Video PID (VideoPid).</p>
    #[serde(rename = "PcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    /// <p>Specify the number of milliseconds between instances of the program map table (PMT) in the output transport stream.</p>
    #[serde(rename = "PmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Specify the packet identifier (PID) for the program map table (PMT) itself. Default is 480.</p>
    #[serde(rename = "PmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    /// <p>Specify the packet identifier (PID) of the private metadata stream. Default is 503.</p>
    #[serde(rename = "PrivateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    /// <p>Use Program number (programNumber) to specify the program number used in the program map table (PMT) for this output. Default is 1. Program numbers and program map tables are parts of MPEG-2 transport stream containers, used for organizing data.</p>
    #[serde(rename = "ProgramNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>When set to CBR, inserts null packets into transport stream to fill specified bitrate. When set to VBR, the bitrate setting acts as the maximum bitrate, but the output will not be padded up to that bitrate.</p>
    #[serde(rename = "RateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<String>,
    /// <p>Include this in your job settings to put SCTE-35 markers in your HLS and transport stream outputs at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "Scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<M2tsScte35Esam>,
    /// <p>Specify the packet identifier (PID) of the SCTE-35 stream in the transport stream.</p>
    #[serde(rename = "Scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    /// <p>For SCTE-35 markers from your input-- Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want SCTE-35 markers in this output. For SCTE-35 markers from an ESAM XML document-- Choose None (NONE). Also provide the ESAM XML as a string in the setting Signal processing notification XML (sccXml). Also enable ESAM SCTE-35 (include the property scte35Esam).</p>
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    /// <p>Inserts segmentation markers at each segmentation<em>time period. rai</em>segstart sets the Random Access Indicator bit in the adaptation field. rai<em>adapt sets the RAI bit and adds the current timecode in the private data bytes. psi</em>segstart inserts PAT and PMT tables at the start of segments. ebp adds Encoder Boundary Point information to the adaptation field as per OpenCable specification OC-SP-EBP-I01-130118. ebp_legacy adds Encoder Boundary Point information to the adaptation field using a legacy proprietary format.</p>
    #[serde(rename = "SegmentationMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<String>,
    /// <p>The segmentation style parameter controls how segmentation markers are inserted into the transport stream. With avails, it is possible that segments may be truncated, which can influence where future segmentation markers are inserted. When a segmentation style of &quot;reset<em>cadence&quot; is selected and a segment is truncated due to an avail, we will reset the segmentation cadence. This means the subsequent segment will have a duration of of $segmentation</em>time seconds. When a segmentation style of &quot;maintain<em>cadence&quot; is selected and a segment is truncated due to an avail, we will not reset the segmentation cadence. This means the subsequent segment will likely be truncated as well. However, all segments after that will have a duration of $segmentation</em>time seconds. Note that EBP lookahead is a slight exception to this rule.</p>
    #[serde(rename = "SegmentationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<String>,
    /// <p>Specify the length, in seconds, of each segment. Required unless markers is set to <em>none</em>.</p>
    #[serde(rename = "SegmentationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,
    /// <p>Specify the packet identifier (PID) for timed metadata in this output. Default is 502.</p>
    #[serde(rename = "TimedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    /// <p>Specify the ID for the transport stream itself in the program map table for this output. Transport stream IDs and program map tables are parts of MPEG-2 transport stream containers, used for organizing data.</p>
    #[serde(rename = "TransportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Specify the packet identifier (PID) of the elementary video stream in the transport stream.</p>
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
    /// <p>If INSERT, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "NielsenId3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "PatInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    /// <p>When set to PCR<em>EVERY</em>PES_PACKET a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This parameter is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
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
    /// <p>For SCTE-35 markers from your input-- Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want SCTE-35 markers in this output. For SCTE-35 markers from an ESAM XML document-- Choose None (NONE) if you don&#39;t want manifest conditioning. Choose Passthrough (PASSTHROUGH) and choose Ad markers (adMarkers) if you do want manifest conditioning. In both cases, also provide the ESAM XML as a string in the setting Signal processing notification XML (sccXml).</p>
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    /// <p>Applies only to HLS outputs. Use this setting to specify whether the service inserts the ID3 timed metadata from the input in this output.</p>
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

/// <p>Overlay motion graphics on top of your video at the time that you specify.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionImageInserter {
    /// <p>If your motion graphic asset is a .mov file, keep this setting unspecified. If your motion graphic asset is a series of .png files, specify the frame rate of the overlay in frames per second, as a fraction. For example, specify 24 fps as 24/1. Make sure that the number of images in your series matches the frame rate and your intended overlay duration. For example, if you want a 30-second overlay at 30 fps, you should have 900 .png images. This overlay frame rate doesn&#39;t need to match the frame rate of the underlying video.</p>
    #[serde(rename = "Framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<MotionImageInsertionFramerate>,
    /// <p>Specify the .mov file or series of .png files that you want to overlay on your video. For .png files, provide the file name of the first file in the series. Make sure that the names of the .png files end with sequential numbers that specify the order that they are played in. For example, overlay<em>000.png, overlay</em>001.png, overlay<em>002.png, and so on. The sequence must start at zero, and each image file name must have the same number of digits. Pad your initial file names with enough zeros to complete the sequence. For example, if the first image is overlay</em>0.png, there can be only 10 images in the sequence, with the last image being overlay<em>9.png. But if the first image is overlay</em>00.png, there can be 100 images in the sequence.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>Choose the type of motion graphic asset that you are providing for your overlay. You can choose either a .mov file or a series of .png files.</p>
    #[serde(rename = "InsertionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion_mode: Option<String>,
    /// <p>Use Offset to specify the placement of your motion graphic overlay on the video frame. Specify in pixels, from the upper-left corner of the frame. If you don&#39;t specify an offset, the service scales your overlay to the full size of the frame. Otherwise, the service inserts the overlay at its native resolution and scales the size up or down with any video scaling.</p>
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<MotionImageInsertionOffset>,
    /// <p>Specify whether your motion graphic overlay repeats on a loop or plays only once.</p>
    #[serde(rename = "Playback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback: Option<String>,
    /// <p>Specify when the motion overlay begins. Use timecode format (HH:MM:SS:FF or HH:MM:SS;FF). Make sure that the timecode you provide here takes into account how you have set up your timecode configuration under both job settings and input settings. The simplest way to do that is to set both to start at 0. If you need to set up your job to follow timecodes embedded in your source that don&#39;t start at zero, make sure that you specify a start time that is after the first embedded timecode. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/setting-up-timecode.html Find job-wide and input timecode configuration settings in your JSON job settings specification at settings&gt;timecodeConfig&gt;source and settings&gt;inputs&gt;timecodeSource.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// <p>For motion overlays that don&#39;t have a built-in frame rate, specify the frame rate of the overlay in frames per second, as a fraction. For example, specify 24 fps as 24/1. The overlay frame rate doesn&#39;t need to match the frame rate of the underlying video.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionImageInsertionFramerate {
    /// <p>The bottom of the fraction that expresses your overlay frame rate. For example, if your frame rate is 24 fps, set this value to 1.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>The top of the fraction that expresses your overlay frame rate. For example, if your frame rate is 24 fps, set this value to 24.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
}

/// <p>Specify the offset between the upper-left corner of the video frame and the top left corner of the overlay.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionImageInsertionOffset {
    /// <p>Set the distance, in pixels, between the overlay and the left edge of the video frame.</p>
    #[serde(rename = "ImageX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i64>,
    /// <p>Set the distance, in pixels, between the overlay and the top edge of the video frame.</p>
    #[serde(rename = "ImageY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i64>,
}

/// <p>Settings for MOV Container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovSettings {
    /// <p>When enabled, include &#39;clap&#39; atom if appropriate for the video output settings.</p>
    #[serde(rename = "ClapAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clap_atom: Option<String>,
    /// <p>When enabled, file composition times will start at zero, composition times in the &#39;ctts&#39; (composition time to sample) box for B-frames will be negative, and a &#39;cslg&#39; (composition shift least greatest) box will be included per 14496-1 amendment 1. This improves compatibility with Apple players and tools.</p>
    #[serde(rename = "CslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    /// <p>When set to XDCAM, writes MPEG2 video streams into the QuickTime file using XDCAM fourcc codes. This increases compatibility with Apple editors and players, but may decrease compatibility with other players. Only applicable when the video codec is MPEG2.</p>
    #[serde(rename = "Mpeg2FourCCControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_four_cc_control: Option<String>,
    /// <p>If set to OMNEON, inserts Omneon-compatible padding</p>
    #[serde(rename = "PaddingControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_control: Option<String>,
    /// <p>Always keep the default value (SELF_CONTAINED) for this setting.</p>
    #[serde(rename = "Reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value MP2.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp2Settings {
    /// <p>Specify the average bitrate in bits per second.</p>
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

/// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value MP3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp3Settings {
    /// <p>Specify the average bitrate in bits per second.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Choosing Mono on the console gives you 1 output channel; choosing Stereo gives you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Specify whether the service encodes this MP3 audio output with a constant bitrate (CBR) or a variable bitrate (VBR).</p>
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Required when you set Bitrate control mode (rateControlMode) to VBR. Specify the audio quality of this MP3 output from 0 (highest quality) to 9 (lowest quality).</p>
    #[serde(rename = "VbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<i64>,
}

/// <p>Settings for MP4 container. You can create audio-only AAC outputs with this container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp4Settings {
    /// <p>When enabled, file composition times will start at zero, composition times in the &#39;ctts&#39; (composition time to sample) box for B-frames will be negative, and a &#39;cslg&#39; (composition shift least greatest) box will be included per 14496-1 amendment 1. This improves compatibility with Apple players and tools.</p>
    #[serde(rename = "CslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    /// <p>Ignore this setting unless compliance to the CTTS box version specification matters in your workflow. Specify a value of 1 to set your CTTS box version to 1 and make your output compliant with the specification. When you specify a value of 1, you must also set CSLG atom (cslgAtom) to the value INCLUDE. Keep the default value 0 to set your CTTS box version to 0. This can provide backward compatibility for some players and packagers.</p>
    #[serde(rename = "CttsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctts_version: Option<i64>,
    /// <p>Inserts a free-space box immediately after the moov box.</p>
    #[serde(rename = "FreeSpaceBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space_box: Option<String>,
    /// <p>If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.</p>
    #[serde(rename = "MoovPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moov_placement: Option<String>,
    /// <p>Overrides the &quot;Major Brand&quot; field in the output file. Usually not necessary to specify.</p>
    #[serde(rename = "Mp4MajorBrand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_major_brand: Option<String>,
}

/// <p>Settings for MP4 segments in DASH</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MpdSettings {
    /// <p>Use this setting only in DASH output groups that include sidecar TTML or IMSC captions.  You specify sidecar captions in a separate output from your audio and video. Choose Raw (RAW) for captions in a single XML file in a raw container. Choose Fragmented MPEG-4 (FRAGMENTED_MP4) for captions in XML format contained within fragmented MP4 files. This set of fragmented MP4 files is separate from your video and audio fragmented MP4 files.</p>
    #[serde(rename = "CaptionContainerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_container_type: Option<String>,
    /// <p>Use this setting only when you specify SCTE-35 markers from ESAM. Choose INSERT to put SCTE-35 markers in this output at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "Scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<String>,
    /// <p>Ignore this setting unless you have SCTE-35 markers in your input video file. Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want those SCTE-35 markers in this output.</p>
    #[serde(rename = "Scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value MPEG2.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mpeg2Settings {
    /// <p>Adaptive quantization. Allows intra-frame quantizers to vary to improve visual quality.</p>
    #[serde(rename = "AdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "Bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Use Level (Mpeg2CodecLevel) to set the MPEG-2 level for the video output.</p>
    #[serde(rename = "CodecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>Use Profile (Mpeg2CodecProfile) to set the MPEG-2 profile for the video output.</p>
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "DynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job sepecification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>When set to INTERPOLATE, produces smoother motion during frame rate conversion.</p>
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Frame rate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Frame rate numerator - frame rate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
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
    /// <p>Indicates if the GOP Size in MPEG2 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
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
    /// <p>Use Interlace mode (InterlaceMode) to choose the scan line type for the output. * Top Field First (TOP<em>FIELD) and Bottom Field First (BOTTOM</em>FIELD) produce interlaced output with the entire output having the same field polarity (top or bottom first). * Follow, Default Top (FOLLOW<em>TOP</em>FIELD) and Follow, Default Bottom (FOLLOW<em>BOTTOM</em>FIELD) use the same field polarity as the source. Therefore, behavior depends on the input scan type.
    /// - If the source is interlaced, the output will be interlaced with the same polarity as the source (it will follow the source). The output could therefore be a mix of &quot;top field first&quot; and &quot;bottom field first&quot;.
    /// - If the source is progressive, the output will be interlaced with &quot;top field first&quot; or &quot;bottom field first&quot; polarity, depending on which of the Follow options you chose.</p>
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Use Intra DC precision (Mpeg2IntraDcPrecision) to set quantization precision for intra-block DC coefficients. If you choose the value auto, the service will automatically select the precision based on the per-frame compression ratio.</p>
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
    /// <p>Using the API, enable ParFollowSource if you want the service to use the pixel aspect ratio from the input. Using the console, do this by choosing Follow source for Pixel aspect ratio.</p>
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
    /// <p>Use Quality tuning level (Mpeg2QualityTuningLevel) to specifiy whether to use single-pass or multipass video encoding.</p>
    #[serde(rename = "QualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Use Rate control mode (Mpeg2RateControlMode) to specifiy whether the bitrate is variable (vbr) or constant (cbr).</p>
    #[serde(rename = "RateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default.</p>
    #[serde(rename = "SceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Enables Slow PAL rate conversion. 23.976fps and 24fps input is relabeled as 25fps, and audio is sped up correspondingly.</p>
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Softness. Selects quantizer matrix, larger values reduce high-frequency content in the encoded image.</p>
    #[serde(rename = "Softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>Adjust quantization within each frame based on spatial variation of content complexity.</p>
    #[serde(rename = "SpatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>Produces a Type D-10 compatible bitstream (SMPTE 356M-2001).</p>
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// <p>Only use Telecine (Mpeg2Telecine) when you set Framerate (Framerate) to 29.970. Set Telecine (Mpeg2Telecine) to Hard (hard) to produce a 29.97i output from a 23.976 input. Set it to Soft (soft) to produce 23.976 output and leave converstion to the player.</p>
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Adjust quantization within each frame based on temporal variation of content complexity.</p>
    #[serde(rename = "TemporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
}

/// <p>Specify the details for each additional Microsoft Smooth Streaming manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsSmoothAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your Microsoft Smooth group is film-name.ismv. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.ismv.</p>
    #[serde(rename = "ManifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "SelectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>If you are using DRM, set DRM System (MsSmoothEncryptionSettings) to specify the value SpekeKeyProvider.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsSmoothEncryptionSettings {
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
}

/// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to MS<em>SMOOTH</em>GROUP_SETTINGS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsSmoothGroupSettings {
    /// <p>By default, the service creates one .ism Microsoft Smooth Streaming manifest for each Microsoft Smooth Streaming output group in your job. This default manifest references every output in the output group. To create additional manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "AdditionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<MsSmoothAdditionalManifest>>,
    /// <p>COMBINE<em>DUPLICATE</em>STREAMS combines identical audio encoding settings across a Microsoft Smooth output group into a single audio stream.</p>
    #[serde(rename = "AudioDeduplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_deduplication: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>If you are using DRM, set DRM System (MsSmoothEncryptionSettings) to specify the value SpekeKeyProvider.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MsSmoothEncryptionSettings>,
    /// <p>Use Fragment length (FragmentLength) to specify the mp4 fragment sizes in seconds. Fragment length must be compatible with GOP size and frame rate.</p>
    #[serde(rename = "FragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Use Manifest encoding (MsSmoothManifestEncoding) to specify the encoding format for the server and client manifest. Valid options are utf8 and utf16.</p>
    #[serde(rename = "ManifestEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_encoding: Option<String>,
}

/// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NielsenConfiguration {
    /// <p>Nielsen has discontinued the use of breakout code functionality. If you must include this property, set the value to zero.</p>
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
    /// <p>Use Noise reducer filter (NoiseReducerFilter) to select one of the following spatial image filtering functions. To use this setting, you must also enable Noise reducer (NoiseReducer). * Bilateral preserves edges while reducing noise. * Mean (softest), Gaussian, Lanczos, and Sharpen (sharpest) do convolution filtering. * Conserve does min/max noise reduction. * Spatial does frequency-domain filtering based on JND principles. * Temporal optimizes video quality for complex motion.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p>Settings for a noise reducer filter</p>
    #[serde(rename = "FilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<NoiseReducerFilterSettings>,
    /// <p>Noise reducer filter settings for spatial filter.</p>
    #[serde(rename = "SpatialFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_filter_settings: Option<NoiseReducerSpatialFilterSettings>,
    /// <p>Noise reducer filter settings for temporal filter.</p>
    #[serde(rename = "TemporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<NoiseReducerTemporalFilterSettings>,
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

/// <p>Noise reducer filter settings for temporal filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseReducerTemporalFilterSettings {
    /// <p>Use Aggressive mode for content that has complex motion. Higher values produce stronger temporal filtering. This filters highly complex scenes more aggressively and creates better VQ for low bitrate outputs.</p>
    #[serde(rename = "AggressiveMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggressive_mode: Option<i64>,
    /// <p>The speed of the filter (higher number is faster). Low setting reduces bit rate at the cost of transcode time, high setting improves transcode time at the cost of bit rate.</p>
    #[serde(rename = "Speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// <p>Specify the strength of the noise reducing filter on this output. Higher values produce stronger filtering. We recommend the following value ranges, depending on the result that you want: * 0-2 for complexity reduction with minimal sharpness loss * 2-8 for complexity reduction with image preservation * 8-16 for a high level of complexity reduction</p>
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
    /// <p>Container specific settings.</p>
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
    /// <p>Specific settings for this type of output.</p>
    #[serde(rename = "OutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_settings: Option<OutputSettings>,
    /// <p>Use Preset (Preset) to specifiy a preset for your transcoding settings. Provide the system or custom preset name. You can specify either Preset (Preset) or Container settings (ContainerSettings), but not both.</p>
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// <p>(VideoDescription) contains a group of video encoding settings. The specific video settings depend on the video codec that you choose when you specify a value for Video codec (codec). Include one instance of (VideoDescription) per output.</p>
    #[serde(rename = "VideoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>OutputChannel mapping settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputChannelMapping {
    /// <p>List of input channels</p>
    #[serde(rename = "InputChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_channels: Option<Vec<i64>>,
}

/// <p>Details regarding output</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputDetail {
    /// <p>Duration in milliseconds</p>
    #[serde(rename = "DurationInMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    /// <p>Contains details about the output&#39;s video stream</p>
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
    /// <p>Output Group settings, including type</p>
    #[serde(rename = "OutputGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_settings: Option<OutputGroupSettings>,
    /// <p>This object holds groups of encoding settings, one group of settings per output.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

/// <p>Contains details about the output groups specified in the job settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputGroupDetail {
    /// <p>Details about the output</p>
    #[serde(rename = "OutputDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<Vec<OutputDetail>>,
}

/// <p>Output Group settings, including type</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputGroupSettings {
    /// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to CMAF<em>GROUP</em>SETTINGS. Each output in a CMAF Output Group may only contain a single video, audio, or caption output.</p>
    #[serde(rename = "CmafGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_group_settings: Option<CmafGroupSettings>,
    /// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to DASH<em>ISO</em>GROUP_SETTINGS.</p>
    #[serde(rename = "DashIsoGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_iso_group_settings: Option<DashIsoGroupSettings>,
    /// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to FILE<em>GROUP</em>SETTINGS.</p>
    #[serde(rename = "FileGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_group_settings: Option<FileGroupSettings>,
    /// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to HLS<em>GROUP</em>SETTINGS.</p>
    #[serde(rename = "HlsGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    /// <p>Required when you set (Type) under (OutputGroups)&gt;(OutputGroupSettings) to MS<em>SMOOTH</em>GROUP_SETTINGS.</p>
    #[serde(rename = "MsSmoothGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,
    /// <p>Type of output group (File group, Apple HLS, DASH ISO, Microsoft Smooth Streaming, CMAF)</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Specific settings for this type of output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputSettings {
    /// <p>Settings for HLS output groups</p>
    #[serde(rename = "HlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_settings: Option<HlsSettings>,
}

/// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Settings for preset</p>
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
    /// <p>Container specific settings.</p>
    #[serde(rename = "ContainerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    /// <p>(VideoDescription) contains a group of video encoding settings. The specific video settings depend on the video codec that you choose when you specify a value for Video codec (codec). Include one instance of (VideoDescription) per output.</p>
    #[serde(rename = "VideoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value PRORES.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProresSettings {
    /// <p>Use Profile (ProResCodecProfile) to specifiy the type of Apple ProRes codec to use for this output.</p>
    #[serde(rename = "CodecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job sepecification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "FramerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>When set to INTERPOLATE, produces smoother motion during frame rate conversion.</p>
    #[serde(rename = "FramerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>Frame rate denominator.</p>
    #[serde(rename = "FramerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator.</p>
    #[serde(rename = "FramerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Use Interlace mode (InterlaceMode) to choose the scan line type for the output. * Top Field First (TOP<em>FIELD) and Bottom Field First (BOTTOM</em>FIELD) produce interlaced output with the entire output having the same field polarity (top or bottom first). * Follow, Default Top (FOLLOW<em>TOP</em>FIELD) and Follow, Default Bottom (FOLLOW<em>BOTTOM</em>FIELD) use the same field polarity as the source. Therefore, behavior depends on the input scan type.
    /// - If the source is interlaced, the output will be interlaced with the same polarity as the source (it will follow the source). The output could therefore be a mix of &quot;top field first&quot; and &quot;bottom field first&quot;.
    /// - If the source is progressive, the output will be interlaced with &quot;top field first&quot; or &quot;bottom field first&quot; polarity, depending on which of the Follow options you chose.</p>
    #[serde(rename = "InterlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Use (ProresParControl) to specify how the service determines the pixel aspect ratio. Set to Follow source (INITIALIZE<em>FROM</em>SOURCE) to use the pixel aspect ratio from the input.  To specify a different pixel aspect ratio: Using the console, choose it from the dropdown menu. Using the API, set ProresParControl to (SPECIFIED) and provide  for (ParNumerator) and (ParDenominator).</p>
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
    /// <p>Enables Slow PAL rate conversion. 23.976fps and 24fps input is relabeled as 25fps, and audio is sped up correspondingly.</p>
    #[serde(rename = "SlowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Only use Telecine (ProresTelecine) when you set Framerate (Framerate) to 29.970. Set Telecine (ProresTelecine) to Hard (hard) to produce a 29.97i output from a 23.976 input. Set it to Soft (soft) to produce 23.976 output and leave converstion to the player.</p>
    #[serde(rename = "Telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
}

/// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Queue {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp in epoch seconds for when you created the queue.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description that you create for each queue.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds for when you most recently updated the queue.</p>
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name that you create for each queue. Each name must be unique within your account.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment.</p>
    #[serde(rename = "PricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,
    /// <p>The estimated number of jobs with a PROGRESSING status.</p>
    #[serde(rename = "ProgressingJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progressing_jobs_count: Option<i64>,
    /// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
    #[serde(rename = "ReservationPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan: Option<ReservationPlan>,
    /// <p>Queues can be ACTIVE or PAUSED. If you pause a queue, the service won&#39;t begin processing jobs in that queue. Jobs that are running when you pause the queue continue to run until they finish or result in an error.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The estimated number of jobs with a SUBMITTED status.</p>
    #[serde(rename = "SubmittedJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_jobs_count: Option<i64>,
    /// <p>Specifies whether this on-demand queue is system or custom. System queues are built in. You can&#39;t modify or delete system queues. You can create and modify custom queues.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Use Rectangle to identify a specific area of the video frame.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    /// <p>Height of rectangle in pixels. Specify only even numbers.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Width of rectangle in pixels. Specify only even numbers.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// <p>The distance, in pixels, between the rectangle and the left edge of the video frame. Specify only even numbers.</p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// <p>The distance, in pixels, between the rectangle and the top edge of the video frame. Specify only even numbers.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

/// <p>Use Manual audio remixing (RemixSettings) to adjust audio levels for each audio channel in each output of your job. With audio remixing, you can output more or fewer audio channels than your input audio source provides.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemixSettings {
    /// <p>Channel mapping (ChannelMapping) contains the group of fields that hold the remixing value for each channel. Units are in dB. Acceptable values are within the range from -60 (mute) through 6. A setting of 0 passes the input channel unchanged to the output channel (no attenuation or amplification).</p>
    #[serde(rename = "ChannelMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_mapping: Option<ChannelMapping>,
    /// <p>Specify the number of audio channels from your input that you want to use in your output. With remixing, you might combine or split the data in these channels, so the number of channels in your final output might be different.</p>
    #[serde(rename = "ChannelsIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_in: Option<i64>,
    /// <p>Specify the number of channels in this output after remixing. Valid values: 1, 2, 4, 6, 8... 64. (1 and even numbers to 64.)</p>
    #[serde(rename = "ChannelsOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_out: Option<i64>,
}

/// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPlan {
    /// <p>The length of the term of your reserved queue pricing plan commitment.</p>
    #[serde(rename = "Commitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    /// <p>The timestamp in epoch seconds for when the current pricing plan term for this reserved queue expires.</p>
    #[serde(rename = "ExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>The timestamp in epoch seconds for when you set up the current pricing plan for this reserved queue.</p>
    #[serde(rename = "PurchasedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_at: Option<f64>,
    /// <p>Specifies whether the term of your reserved queue pricing plan is automatically extended (AUTO_RENEW) or expires (EXPIRE) at the end of the term.</p>
    #[serde(rename = "RenewalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_type: Option<String>,
    /// <p>Specifies the number of reserved transcode slots (RTS) for this queue. The number of RTS determines how many jobs the queue can process in parallel; each RTS can process one job at a time. When you increase this number, you extend your existing commitment with a new 12-month commitment for a larger number of RTS. The new commitment begins when you purchase the additional capacity. You can&#39;t decrease the number of RTS in your reserved queue.</p>
    #[serde(rename = "ReservedSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_slots: Option<i64>,
    /// <p>Specifies whether the pricing plan for your reserved queue is ACTIVE or EXPIRED.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReservationPlanSettings {
    /// <p>The length of the term of your reserved queue pricing plan commitment.</p>
    #[serde(rename = "Commitment")]
    pub commitment: String,
    /// <p>Specifies whether the term of your reserved queue pricing plan is automatically extended (AUTO_RENEW) or expires (EXPIRE) at the end of the term. When your term is auto renewed, you extend your commitment by 12 months from the auto renew date. You can cancel this commitment.</p>
    #[serde(rename = "RenewalType")]
    pub renewal_type: String,
    /// <p>Specifies the number of reserved transcode slots (RTS) for this queue. The number of RTS determines how many jobs the queue can process in parallel; each RTS can process one job at a time. You can&#39;t decrease the number of RTS in your reserved queue. You can increase the number of RTS by extending your existing commitment with a new 12-month commitment for the larger number. The new commitment begins when you purchase the additional capacity. You can&#39;t cancel your commitment or revert to your original commitment after you increase the capacity.</p>
    #[serde(rename = "ReservedSlots")]
    pub reserved_slots: i64,
}

/// <p>The Amazon Resource Name (ARN) and tags for an AWS Elemental MediaConvert resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don&#39;t use this setting, S3 automatically applies the default access control list PRIVATE.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3DestinationAccessControl {
    /// <p>Choose an Amazon S3 canned ACL for MediaConvert to apply to this output.</p>
    #[serde(rename = "CannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

/// <p>Settings associated with S3 destination</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3DestinationSettings {
    /// <p>Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don&#39;t use this setting, S3 automatically applies the default access control list PRIVATE.</p>
    #[serde(rename = "AccessControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<S3DestinationAccessControl>,
    /// <p>Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.</p>
    #[serde(rename = "Encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<S3EncryptionSettings>,
}

/// <p>Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3EncryptionSettings {
    /// <p>Specify how you want your data keys managed. AWS uses data keys to encrypt your content. AWS also encrypts the data keys themselves, using a customer master key (CMK), and then stores the encrypted data keys alongside your encrypted content. Use this setting to specify which AWS service manages the CMK. For simplest set up, choose Amazon S3 (SERVER<em>SIDE</em>ENCRYPTION<em>S3). If you want your master key to be managed by AWS Key Management Service (KMS), choose AWS KMS (SERVER</em>SIDE<em>ENCRYPTION</em>KMS). By default, when you choose AWS KMS, KMS uses the AWS managed customer master key (CMK) associated with Amazon S3 to encrypt your data keys. You can optionally choose to specify a different, customer managed CMK. Do so by specifying the Amazon Resource Name (ARN) of the key for the setting  KMS ARN (kmsKeyArn).</p>
    #[serde(rename = "EncryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Optionally, specify the customer master key (CMK) that you want to use to encrypt the data key that AWS uses to encrypt your output content. Enter the Amazon Resource Name (ARN) of the CMK. To use this setting, you must also set Server-side encryption (S3ServerSideEncryptionType) to AWS KMS (SERVER<em>SIDE</em>ENCRYPTION_KMS). If you set Server-side encryption to AWS KMS but don&#39;t specify a CMK here, AWS uses the AWS managed CMK associated with Amazon S3.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Settings for SCC caption output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SccDestinationSettings {
    /// <p>Set Framerate (SccDestinationFramerate) to make sure that the captions and the video are synchronized in the output. Specify a frame rate that matches the frame rate of the associated video. If the video frame rate is 29.97, choose 29.97 dropframe (FRAMERATE<em>29</em>97<em>DROPFRAME) only if the video has video</em>insertion=true and drop<em>frame</em>timecode=true; otherwise, choose 29.97 non-dropframe (FRAMERATE<em>29</em>97<em>NON</em>DROPFRAME).</p>
    #[serde(rename = "Framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<String>,
}

/// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProvider {
    /// <p>If you want your key provider to encrypt the content keys that it provides to MediaConvert, set up a certificate with a master key using AWS Certificate Manager. Specify the certificate&#39;s Amazon Resource Name (ARN) here.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Specify the resource ID that your SPEKE-compliant key provider uses to identify this content.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>Relates to SPEKE implementation. DRM system identifiers. DASH output groups support a max of two system ids. Other group types support one system id. See
    /// https://dashif.org/identifiers/content_protection/ for more details.</p>
    #[serde(rename = "SystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_ids: Option<Vec<String>>,
    /// <p>Specify the URL to the key server that your SPEKE-compliant DRM key provider uses to provide keys for encrypting your content.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>If your output group type is CMAF, use these settings when doing DRM encryption with a SPEKE-compliant key provider. If your output group type is HLS, DASH, or Microsoft Smooth, use the SpekeKeyProvider settings instead.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpekeKeyProviderCmaf {
    /// <p>If you want your key provider to encrypt the content keys that it provides to MediaConvert, set up a certificate with a master key using AWS Certificate Manager. Specify the certificate&#39;s Amazon Resource Name (ARN) here.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Specify the DRM system IDs that you want signaled in the DASH manifest that MediaConvert creates as part of this CMAF package. The DASH manifest can currently signal up to three system IDs. For more information, see https://dashif.org/identifiers/content_protection/.</p>
    #[serde(rename = "DashSignaledSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_signaled_system_ids: Option<Vec<String>>,
    /// <p>Specify the DRM system ID that you want signaled in the HLS manifest that MediaConvert creates as part of this CMAF package. The HLS manifest can currently signal only one system ID. For more information, see https://dashif.org/identifiers/content_protection/.</p>
    #[serde(rename = "HlsSignaledSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_signaled_system_ids: Option<Vec<String>>,
    /// <p>Specify the resource ID that your SPEKE-compliant key provider uses to identify this content.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>Specify the URL to the key server that your SPEKE-compliant DRM key provider uses to provide keys for encrypting your content.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Use these settings to set up encryption with a static key provider.</p>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_value: Option<String>,
    /// <p>Relates to DRM implementation. The location of the license server used for protecting content.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Settings for Teletext caption output</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeletextDestinationSettings {
    /// <p>Set pageNumber to the Teletext page number for the destination captions for this output. This value must be a three-digit hexadecimal string; strings ending in -FF are invalid. If you are passing through the entire set of Teletext data, do not use this field.</p>
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// <p>Specify the page types for this Teletext page. If you don&#39;t specify a value here, the service sets the page type to the default value Subtitle (PAGE<em>TYPE</em>SUBTITLE). If you pass through the entire set of Teletext data, don&#39;t use this field. When you pass through a set of Teletext pages, your output has the same page types as your input.</p>
    #[serde(rename = "PageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_types: Option<Vec<String>>,
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
    /// <p>Use Position (Position) under under Timecode burn-in (TimecodeBurnIn) to specify the location the burned-in timecode on output video.</p>
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
    /// <p>If you use an editing platform that relies on an anchor timecode, use Anchor Timecode (Anchor) to specify a timecode that will match the input video frame to the output video frame. Use 24-hour format with frame number, (HH:MM:SS:FF) or (HH:MM:SS;FF). This setting ignores frame rate conversion. System behavior for Anchor Timecode varies depending on your setting for Source (TimecodeSource). * If Source (TimecodeSource) is set to Specified Start (SPECIFIEDSTART), the first input frame is the specified value in Start Timecode (Start). Anchor Timecode (Anchor) and Start Timecode (Start) are used calculate output timecode. * If Source (TimecodeSource) is set to Start at 0 (ZEROBASED)  the  first frame is 00:00:00:00. * If Source (TimecodeSource) is set to Embedded (EMBEDDED), the  first frame is the timecode value on the first input frame of the input.</p>
    #[serde(rename = "Anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// <p>Use Source (TimecodeSource) to set how timecodes are handled within this job. To make sure that your video, audio, captions, and markers are synchronized and that time-based features, such as image inserter, work correctly, choose the Timecode source option that matches your assets. All timecodes are in a 24-hour format with frame number (HH:MM:SS:FF). * Embedded (EMBEDDED) - Use the timecode that is in the input video. If no embedded timecode is in the source, the service will use Start at 0 (ZEROBASED) instead. * Start at 0 (ZEROBASED) - Set the timecode of the initial frame to 00:00:00:00. * Specified Start (SPECIFIEDSTART) - Set the timecode of the initial frame to a value other than zero. You use Start timecode (Start) to provide this value.</p>
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_3_insertions: Option<Vec<Id3Insertion>>,
}

/// <p>Information about when jobs are submitted, started, and finished is specified in Unix epoch format in seconds.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackSourceSettings {
    /// <p>Use this setting to select a single captions track from a source. Track numbers correspond to the order in the captions source file. For IMF sources, track numbering is based on the order that the captions appear in the CPL. For example, use 1 to select the captions asset that is listed first in the CPL. To include more than one captions track in your job outputs, create multiple input captions selectors. Specify one track per selector.</p>
    #[serde(rename = "TrackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<i64>,
}

/// <p>Settings specific to TTML caption outputs, including Pass style information (TtmlStylePassthrough).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtmlDestinationSettings {
    /// <p>Pass through style and position information from a TTML-like input source (TTML, SMPTE-TT, CFF-TT) to the CFF-TT output or TTML output.</p>
    #[serde(rename = "StylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to remove tags from. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The keys of the tags that you want to remove from the resource.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJobTemplateRequest {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
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
    /// <p>Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The new queue for the job template, if you are changing it.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<JobTemplateSettings>,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "JobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Settings for preset</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PresetSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "Preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateQueueRequest {
    /// <p>The new description for the queue, if you are changing it.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue that you are modifying.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The new details of your pricing plan for your reserved queue. When you set up a new pricing plan to replace an expired one, you enter into another 12-month commitment. When you add capacity to your queue by increasing the number of RTS, you extend the term of your commitment to 12 months from when you add capacity. After you make these commitments, you can&#39;t cancel them.</p>
    #[serde(rename = "ReservationPlanSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan_settings: Option<ReservationPlanSettings>,
    /// <p>Pause or activate a queue by changing its status between ACTIVE and PAUSED. If you pause a queue, jobs in that queue won&#39;t begin. Jobs that are running when you pause the queue continue to run until they finish or result in an error.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Video codec settings, (CodecSettings) under (VideoDescription), contains the group of settings related to video encoding. The settings in this group vary depending on the value that you choose for Video codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * FRAME<em>CAPTURE, FrameCaptureSettings * H</em>264, H264Settings * H_265, H265Settings * MPEG2, Mpeg2Settings * PRORES, ProresSettings</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoCodecSettings {
    /// <p>Specifies the video codec. This must be equal to one of the enum values defined by the object  VideoCodec.</p>
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value FRAME_CAPTURE.</p>
    #[serde(rename = "FrameCaptureSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value H_264.</p>
    #[serde(rename = "H264Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,
    /// <p>Settings for H265 codec</p>
    #[serde(rename = "H265Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value MPEG2.</p>
    #[serde(rename = "Mpeg2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_settings: Option<Mpeg2Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value PRORES.</p>
    #[serde(rename = "ProresSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prores_settings: Option<ProresSettings>,
}

/// <p>Settings for video outputs</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoDescription {
    /// <p>This setting only applies to H.264, H.265, and MPEG2 outputs. Use Insert AFD signaling (AfdSignaling) to specify whether the service includes AFD values in the output video data and what those values are. * Choose None to remove all AFD values from this output. * Choose Fixed to ignore input AFD values and instead encode the value specified in the job. * Choose Auto to calculate output AFD values based on the input AFD scaler data.</p>
    #[serde(rename = "AfdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>The anti-alias filter is automatically applied to all outputs. The service no longer accepts the value DISABLED for AntiAlias. If you specify that in your job, the service will ignore the setting.</p>
    #[serde(rename = "AntiAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_alias: Option<String>,
    /// <p>Video codec settings, (CodecSettings) under (VideoDescription), contains the group of settings related to video encoding. The settings in this group vary depending on the value that you choose for Video codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * FRAME<em>CAPTURE, FrameCaptureSettings * H</em>264, H264Settings * H_265, H265Settings * MPEG2, Mpeg2Settings * PRORES, ProresSettings</p>
    #[serde(rename = "CodecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<VideoCodecSettings>,
    /// <p>Choose Insert (INSERT) for this setting to include color metadata in this output. Choose Ignore (IGNORE) to exclude color metadata from this output. If you don&#39;t specify a value, the service sets this to Insert by default.</p>
    #[serde(rename = "ColorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame.</p>
    #[serde(rename = "Crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Applies only to 29.97 fps outputs. When this feature is enabled, the service will use drop-frame timecode on outputs. If it is not possible to use drop-frame timecode, the system will fall back to non-drop-frame. This setting is enabled by default when Timecode insertion (TimecodeInsertion) is enabled.</p>
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
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black.</p>
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Respond to AFD (RespondToAfd) to specify how the service changes the video itself in response to AFD values in the input. * Choose Respond to clip the input video frame according to the AFD value, input display aspect ratio, and output display aspect ratio. * Choose Passthrough to include the input AFD values. Do not choose this when AfdSignaling is set to (NONE). A preferred implementation of this workflow is to set RespondToAfd to (NONE) and set AfdSignaling to (AUTO). * Choose None to remove all input AFD values from this output.</p>
    #[serde(rename = "RespondToAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<String>,
    /// <p>Specify how the service handles outputs that have a different aspect ratio from the input aspect ratio. Choose Stretch to output (STRETCH<em>TO</em>OUTPUT) to have the service stretch your video image to fit. Keep the setting Default (DEFAULT) to have the service letterbox your video instead. This setting overrides any value that you specify for the setting Selection placement (position) in this output.</p>
    #[serde(rename = "ScalingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<String>,
    /// <p>Use Sharpness (Sharpness) setting to specify the strength of anti-aliasing. This setting changes the width of the anti-alias filter kernel used for scaling. Sharpness only applies if your output resolution is different from your input resolution. 0 is the softest setting, 100 the sharpest, and 50 recommended for most content.</p>
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i64>,
    /// <p>Applies only to H.264, H.265, MPEG2, and ProRes outputs. Only enable Timecode insertion when the input frame rate is identical to the output frame rate. To include timecodes in this output, set Timecode insertion (VideoTimecodeInsertion) to PIC<em>TIMING</em>SEI. To leave them out, set it to DISABLED. Default is DISABLED. When the service inserts timecodes in an output, by default, it uses any embedded timecodes from the input. If none are present, the service will set the timecode for the first output frame to zero. To change this default behavior, adjust the settings under Timecode configuration (TimecodeConfig). In the console, these settings are located under Job &gt; Job settings &gt; Timecode configuration. Note - Timecode source under input settings (InputTimecodeSource) does not affect the timecodes that are inserted in the output. Source under Job settings &gt; Timecode configuration (TimecodeSource) does.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Enable Dolby Vision feature to produce Dolby Vision compatible video output.</p>
    #[serde(rename = "DolbyVision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_vision: Option<DolbyVision>,
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
    /// <p>Ignore this setting unless this input is a QuickTime animation with an alpha channel. Use this setting to create separate Key and Fill outputs. In each output, specify which part of the input MediaConvert uses. Leave this setting at the default value DISCARD to delete the alpha channel and preserve the video. Set it to REMAP<em>TO</em>LUMA to delete the video and map the alpha channel to the luma channel of your outputs.</p>
    #[serde(rename = "AlphaBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_behavior: Option<String>,
    /// <p>If your input video has accurate color space metadata, or if you don&#39;t know about color space, leave this set to the default value Follow (FOLLOW). The service will automatically detect your input color space. If your input video has metadata indicating the wrong color space, specify the accurate color space here. If your input video is HDR 10 and the SMPTE ST 2086 Mastering Display Color Volume static metadata isn&#39;t present in your video stream, or if that metadata is present but not accurate, choose Force HDR 10 (FORCE_HDR10) here and specify correct values in the input HDR 10 metadata (Hdr10Metadata) settings. For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
    #[serde(rename = "ColorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// <p>There are two sources for color metadata, the input file and the job input settings Color space (ColorSpace) and HDR master display information settings(Hdr10Metadata). The Color space usage setting determines which takes precedence. Choose Force (FORCE) to use color metadata from the input job settings. If you don&#39;t specify values for those settings, the service defaults to using metadata from your input. FALLBACK - Choose Fallback (FALLBACK) to use color metadata from the source when it is present. If there&#39;s no color metadata in your input file, the service defaults to using values you specify in the input settings.</p>
    #[serde(rename = "ColorSpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<String>,
    /// <p>Use these settings to provide HDR 10 metadata that is missing or inaccurate in your input video. Appropriate values vary depending on the input video and must be provided by a color grader. The color grader generates these values during the HDR 10 mastering process. The valid range for each of these settings is 0 to 50,000. Each increment represents 0.00002 in CIE1931 color coordinate. Related settings - When you specify these values, you must also set Color space (ColorSpace) to HDR 10 (HDR10). To specify whether the the values you specify here take precedence over the values in the metadata of your input file, set Color space usage (ColorSpaceUsage). To specify whether color metadata is included in an output, set Color metadata (ColorMetadata). For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
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
    /// <p>Use Rotate (InputRotate) to specify how the service rotates your video. You can choose automatic rotation or specify a rotation. You can specify a clockwise rotation of 0, 90, 180, or 270 degrees. If your input video container is .mov or .mp4 and your input has rotation metadata, you can choose Automatic to have the service rotate your video according to the rotation specified in the metadata. The rotation must be within one degree of 90, 180, or 270 degrees. If the rotation metadata specifies any other rotation, the service will default to no rotation. By default, the service does no rotation, even if your input video has rotation metadata. The service doesn&#39;t pass through rotation metadata.</p>
    #[serde(rename = "Rotate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value WAV.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WavSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "BitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Valid values are 1 and even numbers up to 64. For example, 1, 2, 4, 6, and so on, up to 64.</p>
    #[serde(rename = "Channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>The service defaults to using RIFF for WAV outputs. If your output audio is likely to exceed 4 GB in file size, or if you otherwise need the extended support of the RF64 format, set your output WAV file format to RF64.</p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sample rate in Hz.</p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// Errors returned by AssociateCertificate
#[derive(Debug, PartialEq)]
pub enum AssociateCertificateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl AssociateCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AssociateCertificateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateCertificateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AssociateCertificateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AssociateCertificateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociateCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AssociateCertificateError::TooManyRequests(
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
impl fmt::Display for AssociateCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateCertificateError::BadRequest(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::Forbidden(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateCertificateError {}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CancelJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CancelJobError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CancelJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CancelJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CancelJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CancelJobError::Conflict(ref cause) => write!(f, "{}", cause),
            CancelJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CancelJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CancelJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateJobError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateJobError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobError {}
/// Errors returned by CreateJobTemplate
#[derive(Debug, PartialEq)]
pub enum CreateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobTemplateError {}
/// Errors returned by CreatePreset
#[derive(Debug, PartialEq)]
pub enum CreatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreatePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreatePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreatePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreatePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            CreatePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreatePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePresetError {}
/// Errors returned by CreateQueue
#[derive(Debug, PartialEq)]
pub enum CreateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateQueueError {}
/// Errors returned by DeleteJobTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeleteJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJobTemplateError {}
/// Errors returned by DeletePreset
#[derive(Debug, PartialEq)]
pub enum DeletePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeletePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeletePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeletePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeletePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeletePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            DeletePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeletePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePresetError {}
/// Errors returned by DeleteQueue
#[derive(Debug, PartialEq)]
pub enum DeleteQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeleteQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteQueueError {}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeEndpointsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeEndpointsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeEndpointsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEndpointsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointsError {}
/// Errors returned by DisassociateCertificate
#[derive(Debug, PartialEq)]
pub enum DisassociateCertificateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DisassociateCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateCertificateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateCertificateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DisassociateCertificateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DisassociateCertificateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisassociateCertificateError::TooManyRequests(
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
impl fmt::Display for DisassociateCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateCertificateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateCertificateError {}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJobError::BadRequest(err.msg))
                }
                "ConflictException" => return RusotoError::Service(GetJobError::Conflict(err.msg)),
                "ForbiddenException" => {
                    return RusotoError::Service(GetJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetJobError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJobError::Conflict(ref cause) => write!(f, "{}", cause),
            GetJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobError {}
/// Errors returned by GetJobTemplate
#[derive(Debug, PartialEq)]
pub enum GetJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJobTemplateError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobTemplateError {}
/// Errors returned by GetPreset
#[derive(Debug, PartialEq)]
pub enum GetPresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetPresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetPresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetPresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetPresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPresetError::Conflict(ref cause) => write!(f, "{}", cause),
            GetPresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetPresetError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPresetError {}
/// Errors returned by GetQueue
#[derive(Debug, PartialEq)]
pub enum GetQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            GetQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            GetQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQueueError {}
/// Errors returned by ListJobTemplates
#[derive(Debug, PartialEq)]
pub enum ListJobTemplatesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListJobTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJobTemplatesError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListJobTemplatesError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListJobTemplatesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListJobTemplatesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJobTemplatesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListJobTemplatesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobTemplatesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::Conflict(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobTemplatesError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJobsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListJobsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListJobsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJobsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListPresets
#[derive(Debug, PartialEq)]
pub enum ListPresetsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListPresetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPresetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListPresetsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListPresetsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListPresetsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListPresetsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListPresetsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPresetsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPresetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPresetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListPresetsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListPresetsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPresetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListPresetsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListPresetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPresetsError {}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListQueuesError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListQueuesError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListQueuesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListQueuesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListQueuesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListQueuesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueuesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Conflict(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListQueuesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListQueuesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueuesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListTagsForResourceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTagsForResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
                }
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
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TagResourceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(TagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
                }
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
        match *self {
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            TagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UntagResourceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UntagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
                }
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
        match *self {
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateJobTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdateJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJobTemplateError {}
/// Errors returned by UpdatePreset
#[derive(Debug, PartialEq)]
pub enum UpdatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdatePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdatePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdatePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePresetError {}
/// Errors returned by UpdateQueue
#[derive(Debug, PartialEq)]
pub enum UpdateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdateQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateQueueError {}
/// Trait representing the capabilities of the MediaConvert API. MediaConvert clients implement this trait.
#[async_trait]
pub trait MediaConvert {
    /// <p>Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.</p>
    async fn associate_certificate(
        &self,
        input: AssociateCertificateRequest,
    ) -> Result<AssociateCertificateResponse, RusotoError<AssociateCertificateError>>;

    /// <p>Permanently cancel a job. Once you have canceled a job, you can&#39;t start it again.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>>;

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>>;

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> Result<CreateJobTemplateResponse, RusotoError<CreateJobTemplateError>>;

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> Result<CreatePresetResponse, RusotoError<CreatePresetError>>;

    /// <p>Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html</p>
    async fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> Result<CreateQueueResponse, RusotoError<CreateQueueError>>;

    /// <p>Permanently delete a job template you have created.</p>
    async fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> Result<DeleteJobTemplateResponse, RusotoError<DeleteJobTemplateError>>;

    /// <p>Permanently delete a preset you have created.</p>
    async fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> Result<DeletePresetResponse, RusotoError<DeletePresetError>>;

    /// <p>Permanently delete a queue you have created.</p>
    async fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> Result<DeleteQueueResponse, RusotoError<DeleteQueueError>>;

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>>;

    /// <p>Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource.</p>
    async fn disassociate_certificate(
        &self,
        input: DisassociateCertificateRequest,
    ) -> Result<DisassociateCertificateResponse, RusotoError<DisassociateCertificateError>>;

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>>;

    /// <p>Retrieve the JSON for a specific job template.</p>
    async fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> Result<GetJobTemplateResponse, RusotoError<GetJobTemplateError>>;

    /// <p>Retrieve the JSON for a specific preset.</p>
    async fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> Result<GetPresetResponse, RusotoError<GetPresetError>>;

    /// <p>Retrieve the JSON for a specific queue.</p>
    async fn get_queue(
        &self,
        input: GetQueueRequest,
    ) -> Result<GetQueueResponse, RusotoError<GetQueueError>>;

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    async fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> Result<ListJobTemplatesResponse, RusotoError<ListJobTemplatesError>>;

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>>;

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    async fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> Result<ListPresetsResponse, RusotoError<ListPresetsError>>;

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>>;

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Modify one of your existing job templates.</p>
    async fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> Result<UpdateJobTemplateResponse, RusotoError<UpdateJobTemplateError>>;

    /// <p>Modify one of your existing presets.</p>
    async fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> Result<UpdatePresetResponse, RusotoError<UpdatePresetError>>;

    /// <p>Modify one of your existing queues.</p>
    async fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> Result<UpdateQueueResponse, RusotoError<UpdateQueueError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaConvertClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaConvertClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaConvertClient {
        MediaConvertClient { client, region }
    }
}

#[async_trait]
impl MediaConvert for MediaConvertClient {
    /// <p>Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.</p>
    async fn associate_certificate(
        &self,
        input: AssociateCertificateRequest,
    ) -> Result<AssociateCertificateResponse, RusotoError<AssociateCertificateError>> {
        let request_uri = "/2017-08-29/certificates";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<AssociateCertificateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateCertificateError::from_response(response))
        }
    }

    /// <p>Permanently cancel a job. Once you have canceled a job, you can&#39;t start it again.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobError::from_response(response))
        }
    }

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>> {
        let request_uri = "/2017-08-29/jobs";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<CreateJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobError::from_response(response))
        }
    }

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> Result<CreateJobTemplateResponse, RusotoError<CreateJobTemplateError>> {
        let request_uri = "/2017-08-29/jobTemplates";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<CreateJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobTemplateError::from_response(response))
        }
    }

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> Result<CreatePresetResponse, RusotoError<CreatePresetError>> {
        let request_uri = "/2017-08-29/presets";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<CreatePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePresetError::from_response(response))
        }
    }

    /// <p>Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html</p>
    async fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> Result<CreateQueueResponse, RusotoError<CreateQueueError>> {
        let request_uri = "/2017-08-29/queues";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<CreateQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateQueueError::from_response(response))
        }
    }

    /// <p>Permanently delete a job template you have created.</p>
    async fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> Result<DeleteJobTemplateResponse, RusotoError<DeleteJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJobTemplateError::from_response(response))
        }
    }

    /// <p>Permanently delete a preset you have created.</p>
    async fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> Result<DeletePresetResponse, RusotoError<DeletePresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePresetError::from_response(response))
        }
    }

    /// <p>Permanently delete a queue you have created.</p>
    async fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> Result<DeleteQueueResponse, RusotoError<DeleteQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteQueueError::from_response(response))
        }
    }

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
        let request_uri = "/2017-08-29/endpoints";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<DescribeEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEndpointsError::from_response(response))
        }
    }

    /// <p>Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource.</p>
    async fn disassociate_certificate(
        &self,
        input: DisassociateCertificateRequest,
    ) -> Result<DisassociateCertificateResponse, RusotoError<DisassociateCertificateError>> {
        let request_uri = format!("/2017-08-29/certificates/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateCertificateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateCertificateError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific job template.</p>
    async fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> Result<GetJobTemplateResponse, RusotoError<GetJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobTemplateError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific preset.</p>
    async fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> Result<GetPresetResponse, RusotoError<GetPresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPresetError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific queue.</p>
    async fn get_queue(
        &self,
        input: GetQueueRequest,
    ) -> Result<GetQueueResponse, RusotoError<GetQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetQueueError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    async fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> Result<ListJobTemplatesResponse, RusotoError<ListJobTemplatesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobTemplatesError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    async fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> Result<ListPresetsResponse, RusotoError<ListPresetsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListPresetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPresetsError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListQueuesError::from_response(response))
        }
    }

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/2017-08-29/tags/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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

    /// <p>Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/2017-08-29/tags";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/2017-08-29/tags/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Modify one of your existing job templates.</p>
    async fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> Result<UpdateJobTemplateResponse, RusotoError<UpdateJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<UpdateJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJobTemplateError::from_response(response))
        }
    }

    /// <p>Modify one of your existing presets.</p>
    async fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> Result<UpdatePresetResponse, RusotoError<UpdatePresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<UpdatePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePresetError::from_response(response))
        }
    }

    /// <p>Modify one of your existing queues.</p>
    async fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> Result<UpdateQueueResponse, RusotoError<UpdateQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<UpdateQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateQueueError::from_response(response))
        }
    }
}
