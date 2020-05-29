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
/// <p>Information about how AWS Ground Station should configure an antenna for downlink during a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaDownlinkConfig {
    /// <p>Object that describes a spectral <code>Config</code>.</p>
    #[serde(rename = "spectrumConfig")]
    pub spectrum_config: SpectrumConfig,
}

/// <p>Information about how AWS Ground Station should conﬁgure an antenna for downlink demod decode during a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaDownlinkDemodDecodeConfig {
    /// <p>Information about the decode <code>Config</code>.</p>
    #[serde(rename = "decodeConfig")]
    pub decode_config: DecodeConfig,
    /// <p>Information about the demodulation <code>Config</code>.</p>
    #[serde(rename = "demodulationConfig")]
    pub demodulation_config: DemodulationConfig,
    /// <p>Information about the spectral <code>Config</code>.</p>
    #[serde(rename = "spectrumConfig")]
    pub spectrum_config: SpectrumConfig,
}

/// <p>Information about the uplink <code>Config</code> of an antenna.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaUplinkConfig {
    /// <p>Information about the uplink spectral <code>Config</code>.</p>
    #[serde(rename = "spectrumConfig")]
    pub spectrum_config: UplinkSpectrumConfig,
    /// <p>EIRP of the target.</p>
    #[serde(rename = "targetEirp")]
    pub target_eirp: Eirp,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelContactRequest {
    /// <p>UUID of a contact.</p>
    #[serde(rename = "contactId")]
    pub contact_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigIdResponse {
    /// <p>ARN of a <code>Config</code>.</p>
    #[serde(rename = "configArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_arn: Option<String>,
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
}

/// <p>An item in a list of <code>Config</code> objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigListItem {
    /// <p>ARN of a <code>Config</code>.</p>
    #[serde(rename = "configArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_arn: Option<String>,
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
    /// <p>Name of a <code>Config</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Object containing the parameters of a <code>Config</code>.</p> <p>See the subtype definitions for what each type of <code>Config</code> contains.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigTypeData {
    /// <p>Information about how AWS Ground Station should configure an antenna for downlink during a contact.</p>
    #[serde(rename = "antennaDownlinkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antenna_downlink_config: Option<AntennaDownlinkConfig>,
    /// <p>Information about how AWS Ground Station should conﬁgure an antenna for downlink demod decode during a contact.</p>
    #[serde(rename = "antennaDownlinkDemodDecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antenna_downlink_demod_decode_config: Option<AntennaDownlinkDemodDecodeConfig>,
    /// <p>Information about how AWS Ground Station should conﬁgure an antenna for uplink during a contact.</p>
    #[serde(rename = "antennaUplinkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antenna_uplink_config: Option<AntennaUplinkConfig>,
    /// <p>Information about the dataflow endpoint <code>Config</code>.</p>
    #[serde(rename = "dataflowEndpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_config: Option<DataflowEndpointConfig>,
    /// <p>Object that determines whether tracking should be used during a contact executed with this <code>Config</code> in the mission profile. </p>
    #[serde(rename = "trackingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_config: Option<TrackingConfig>,
    /// <p>Information about an uplink echo <code>Config</code>.</p> <p>Parameters from the <code>AntennaUplinkConfig</code>, corresponding to the specified <code>AntennaUplinkConfigArn</code>, are used when this <code>UplinkEchoConfig</code> is used in a contact.</p>
    #[serde(rename = "uplinkEchoConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_echo_config: Option<UplinkEchoConfig>,
}

/// <p>Data describing a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactData {
    /// <p>UUID of a contact.</p>
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>Status of a contact.</p>
    #[serde(rename = "contactStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<String>,
    /// <p>End time of a contact.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Error message of a contact.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Name of a ground station.</p>
    #[serde(rename = "groundStation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station: Option<String>,
    /// <p>Maximum elevation angle of a contact.</p>
    #[serde(rename = "maximumElevation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_elevation: Option<Elevation>,
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_arn: Option<String>,
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "postPassEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_pass_end_time: Option<f64>,
    /// <p>Amount of time prior to contact start you’d like to receive a CloudWatch event indicating an upcoming pass.</p>
    #[serde(rename = "prePassStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_pass_start_time: Option<f64>,
    /// <p>Region of a contact.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>ARN of a satellite.</p>
    #[serde(rename = "satelliteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_arn: Option<String>,
    /// <p>Start time of a contact.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Tags assigned to a contact.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContactIdResponse {
    /// <p>UUID of a contact.</p>
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigRequest {
    /// <p>Parameters of a <code>Config</code>.</p>
    #[serde(rename = "configData")]
    pub config_data: ConfigTypeData,
    /// <p>Name of a <code>Config</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Tags assigned to a <code>Config</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDataflowEndpointGroupRequest {
    /// <p>Endpoint details of each endpoint in the dataflow endpoint group.</p>
    #[serde(rename = "endpointDetails")]
    pub endpoint_details: Vec<EndpointDetails>,
    /// <p>Tags of a dataflow endpoint group.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMissionProfileRequest {
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "contactPostPassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_post_pass_duration_seconds: Option<i64>,
    /// <p>Amount of time prior to contact start you’d like to receive a CloudWatch event indicating an upcoming pass.</p>
    #[serde(rename = "contactPrePassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_pre_pass_duration_seconds: Option<i64>,
    /// <p>A list of lists of ARNs. Each list of ARNs is an edge, with a <i>from</i> <code>Config</code> and a <i>to</i> <code>Config</code>.</p>
    #[serde(rename = "dataflowEdges")]
    pub dataflow_edges: Vec<Vec<String>>,
    /// <p>Smallest amount of time in seconds that you’d like to see for an available contact. AWS Ground Station will not present you with contacts shorter than this duration.</p>
    #[serde(rename = "minimumViableContactDurationSeconds")]
    pub minimum_viable_contact_duration_seconds: i64,
    /// <p>Name of a mission profile.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Tags assigned to a mission profile.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>ARN of a tracking <code>Config</code>.</p>
    #[serde(rename = "trackingConfigArn")]
    pub tracking_config_arn: String,
}

/// <p>Information about a dataflow endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataflowEndpoint {
    /// <p>Socket address of a dataflow endpoint.</p>
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SocketAddress>,
    /// <p>Name of a dataflow endpoint.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Status of a dataflow endpoint.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about the dataflow endpoint <code>Config</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataflowEndpointConfig {
    /// <p>Name of a dataflow endpoint.</p>
    #[serde(rename = "dataflowEndpointName")]
    pub dataflow_endpoint_name: String,
    /// <p>Region of a dataflow endpoint.</p>
    #[serde(rename = "dataflowEndpointRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_region: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataflowEndpointGroupIdResponse {
    /// <p>UUID of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_id: Option<String>,
}

/// <p>Item in a list of <code>DataflowEndpoint</code> groups.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataflowEndpointListItem {
    /// <p>ARN of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_arn: Option<String>,
    /// <p>UUID of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_id: Option<String>,
}

/// <p>Information about the decode <code>Config</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecodeConfig {
    /// <p>Unvalidated JSON of a decode <code>Config</code>.</p>
    #[serde(rename = "unvalidatedJSON")]
    pub unvalidated_json: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigRequest {
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    pub config_id: String,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    pub config_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDataflowEndpointGroupRequest {
    /// <p>UUID of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupId")]
    pub dataflow_endpoint_group_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMissionProfileRequest {
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    pub mission_profile_id: String,
}

/// <p>Information about the demodulation <code>Config</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemodulationConfig {
    /// <p>Unvalidated JSON of a demodulation <code>Config</code>.</p>
    #[serde(rename = "unvalidatedJSON")]
    pub unvalidated_json: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeContactRequest {
    /// <p>UUID of a contact.</p>
    #[serde(rename = "contactId")]
    pub contact_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeContactResponse {
    /// <p>UUID of a contact.</p>
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// <p>Status of a contact.</p>
    #[serde(rename = "contactStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<String>,
    /// <p>End time of a contact.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Error message for a contact.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Ground station for a contact.</p>
    #[serde(rename = "groundStation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station: Option<String>,
    /// <p>Maximum elevation angle of a contact.</p>
    #[serde(rename = "maximumElevation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_elevation: Option<Elevation>,
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_arn: Option<String>,
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "postPassEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_pass_end_time: Option<f64>,
    /// <p>Amount of time prior to contact start you’d like to receive a CloudWatch event indicating an upcoming pass.</p>
    #[serde(rename = "prePassStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_pass_start_time: Option<f64>,
    /// <p>Region of a contact.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>ARN of a satellite.</p>
    #[serde(rename = "satelliteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_arn: Option<String>,
    /// <p>Start time of a contact.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Tags assigned to a contact.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Object that represents EIRP.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eirp {
    /// <p>Units of an EIRP.</p>
    #[serde(rename = "units")]
    pub units: String,
    /// <p>Value of an EIRP.</p>
    #[serde(rename = "value")]
    pub value: f64,
}

/// <p>Elevation angle of the satellite in the sky during a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Elevation {
    /// <p>Elevation angle units.</p>
    #[serde(rename = "unit")]
    pub unit: String,
    /// <p>Elevation angle value.</p>
    #[serde(rename = "value")]
    pub value: f64,
}

/// <p>Information about the endpoint details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDetails {
    /// <p>A dataflow endpoint.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<DataflowEndpoint>,
    /// <p>Endpoint security details.</p>
    #[serde(rename = "securityDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_details: Option<SecurityDetails>,
}

/// <p>Object that describes the frequency.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frequency {
    /// <p>Frequency units.</p>
    #[serde(rename = "units")]
    pub units: String,
    /// <p>Frequency value.</p>
    #[serde(rename = "value")]
    pub value: f64,
}

/// <p>Object that describes the frequency bandwidth.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencyBandwidth {
    /// <p>Frequency bandwidth units.</p>
    #[serde(rename = "units")]
    pub units: String,
    /// <p>Frequency bandwidth value.</p>
    #[serde(rename = "value")]
    pub value: f64,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigRequest {
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    pub config_id: String,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    pub config_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConfigResponse {
    /// <p>ARN of a <code>Config</code> </p>
    #[serde(rename = "configArn")]
    pub config_arn: String,
    /// <p>Data elements in a <code>Config</code>.</p>
    #[serde(rename = "configData")]
    pub config_data: ConfigTypeData,
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    pub config_id: String,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
    /// <p>Name of a <code>Config</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Tags assigned to a <code>Config</code>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDataflowEndpointGroupRequest {
    /// <p>UUID of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupId")]
    pub dataflow_endpoint_group_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDataflowEndpointGroupResponse {
    /// <p>ARN of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_arn: Option<String>,
    /// <p>UUID of a dataflow endpoint group.</p>
    #[serde(rename = "dataflowEndpointGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_id: Option<String>,
    /// <p>Details of a dataflow endpoint.</p>
    #[serde(rename = "endpointsDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_details: Option<Vec<EndpointDetails>>,
    /// <p>Tags assigned to a dataflow endpoint group.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMinuteUsageRequest {
    /// <p>The month being requested, with a value of 1-12.</p>
    #[serde(rename = "month")]
    pub month: i64,
    /// <p>The year being requested, in the format of YYYY.</p>
    #[serde(rename = "year")]
    pub year: i64,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMinuteUsageResponse {
    /// <p>Estimated number of minutes remaining for an account, specific to the month being requested.</p>
    #[serde(rename = "estimatedMinutesRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_minutes_remaining: Option<i64>,
    /// <p>Returns whether or not an account has signed up for the reserved minutes pricing plan, specific to the month being requested.</p>
    #[serde(rename = "isReservedMinutesCustomer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reserved_minutes_customer: Option<bool>,
    /// <p>Total number of reserved minutes allocated, specific to the month being requested.</p>
    #[serde(rename = "totalReservedMinuteAllocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_reserved_minute_allocation: Option<i64>,
    /// <p>Total scheduled minutes for an account, specific to the month being requested.</p>
    #[serde(rename = "totalScheduledMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_scheduled_minutes: Option<i64>,
    /// <p>Upcoming minutes scheduled for an account, specific to the month being requested.</p>
    #[serde(rename = "upcomingMinutesScheduled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upcoming_minutes_scheduled: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMissionProfileRequest {
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    pub mission_profile_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMissionProfileResponse {
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "contactPostPassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_post_pass_duration_seconds: Option<i64>,
    /// <p>Amount of time prior to contact start you’d like to receive a CloudWatch event indicating an upcoming pass.</p>
    #[serde(rename = "contactPrePassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_pre_pass_duration_seconds: Option<i64>,
    /// <p>A list of lists of ARNs. Each list of ARNs is an edge, with a <i>from</i> <code>Config</code> and a <i>to</i> <code>Config</code>.</p>
    #[serde(rename = "dataflowEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_edges: Option<Vec<Vec<String>>>,
    /// <p>Smallest amount of time in seconds that you’d like to see for an available contact. AWS Ground Station will not present you with contacts shorter than this duration.</p>
    #[serde(rename = "minimumViableContactDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_viable_contact_duration_seconds: Option<i64>,
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_arn: Option<String>,
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_id: Option<String>,
    /// <p>Name of a mission profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Region of a mission profile.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Tags assigned to a mission profile.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>ARN of a tracking <code>Config</code>.</p>
    #[serde(rename = "trackingConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_config_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSatelliteRequest {
    /// <p>UUID of a satellite.</p>
    #[serde(rename = "satelliteId")]
    pub satellite_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSatelliteResponse {
    /// <p>A list of ground stations to which the satellite is on-boarded.</p>
    #[serde(rename = "groundStations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_stations: Option<Vec<String>>,
    /// <p>NORAD satellite ID number.</p>
    #[serde(rename = "noradSatelliteID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norad_satellite_id: Option<i64>,
    /// <p>ARN of a satellite.</p>
    #[serde(rename = "satelliteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_arn: Option<String>,
    /// <p>UUID of a satellite.</p>
    #[serde(rename = "satelliteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_id: Option<String>,
}

/// <p>Information about the ground station data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GroundStationData {
    /// <p>UUID of a ground station.</p>
    #[serde(rename = "groundStationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station_id: Option<String>,
    /// <p>Name of a ground station.</p>
    #[serde(rename = "groundStationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station_name: Option<String>,
    /// <p>Ground station Region.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigsRequest {
    /// <p>Maximum number of <code>Configs</code> returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Next token returned in the request of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigsResponse {
    /// <p>List of <code>Config</code> items.</p>
    #[serde(rename = "configList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_list: Option<Vec<ConfigListItem>>,
    /// <p>Next token returned in the response of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListContactsRequest {
    /// <p>End time of a contact.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>Name of a ground station.</p>
    #[serde(rename = "groundStation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station: Option<String>,
    /// <p>Maximum number of contacts returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_arn: Option<String>,
    /// <p>Next token returned in the request of a previous <code>ListContacts</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>ARN of a satellite.</p>
    #[serde(rename = "satelliteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_arn: Option<String>,
    /// <p>Start time of a contact.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>Status of a contact reservation.</p>
    #[serde(rename = "statusList")]
    pub status_list: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListContactsResponse {
    /// <p>List of contacts.</p>
    #[serde(rename = "contactList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list: Option<Vec<ContactData>>,
    /// <p>Next token returned in the response of a previous <code>ListContacts</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDataflowEndpointGroupsRequest {
    /// <p>Maximum number of dataflow endpoint groups returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Next token returned in the request of a previous <code>ListDataflowEndpointGroups</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDataflowEndpointGroupsResponse {
    /// <p>A list of dataflow endpoint groups.</p>
    #[serde(rename = "dataflowEndpointGroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_endpoint_group_list: Option<Vec<DataflowEndpointListItem>>,
    /// <p>Next token returned in the response of a previous <code>ListDataflowEndpointGroups</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGroundStationsRequest {
    /// <p>Maximum number of ground stations returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Next token that can be supplied in the next call to get the next page of ground stations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Satellite ID to retrieve on-boarded ground stations.</p>
    #[serde(rename = "satelliteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGroundStationsResponse {
    /// <p>List of ground stations.</p>
    #[serde(rename = "groundStationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_station_list: Option<Vec<GroundStationData>>,
    /// <p>Next token that can be supplied in the next call to get the next page of ground stations.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMissionProfilesRequest {
    /// <p>Maximum number of mission profiles returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Next token returned in the request of a previous <code>ListMissionProfiles</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMissionProfilesResponse {
    /// <p>List of mission profiles.</p>
    #[serde(rename = "missionProfileList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_list: Option<Vec<MissionProfileListItem>>,
    /// <p>Next token returned in the response of a previous <code>ListMissionProfiles</code> call. Used to get the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSatellitesRequest {
    /// <p>Maximum number of satellites returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Next token that can be supplied in the next call to get the next page of satellites.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSatellitesResponse {
    /// <p>Next token that can be supplied in the next call to get the next page of satellites.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of satellites.</p>
    #[serde(rename = "satellites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellites: Option<Vec<SatelliteListItem>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>ARN of a resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Tags assigned to a resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MissionProfileIdResponse {
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_id: Option<String>,
}

/// <p>Item in a list of mission profiles.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MissionProfileListItem {
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_arn: Option<String>,
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_profile_id: Option<String>,
    /// <p>Name of a mission profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Region of a mission profile.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReserveContactRequest {
    /// <p>End time of a contact.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>Name of a ground station.</p>
    #[serde(rename = "groundStation")]
    pub ground_station: String,
    /// <p>ARN of a mission profile.</p>
    #[serde(rename = "missionProfileArn")]
    pub mission_profile_arn: String,
    /// <p>ARN of a satellite</p>
    #[serde(rename = "satelliteArn")]
    pub satellite_arn: String,
    /// <p>Start time of a contact.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>Tags assigned to a contact.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Item in a list of satellites.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SatelliteListItem {
    /// <p>A list of ground stations to which the satellite is on-boarded.</p>
    #[serde(rename = "groundStations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_stations: Option<Vec<String>>,
    /// <p>NORAD satellite ID number.</p>
    #[serde(rename = "noradSatelliteID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norad_satellite_id: Option<i64>,
    /// <p>ARN of a satellite.</p>
    #[serde(rename = "satelliteArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_arn: Option<String>,
    /// <p>UUID of a satellite.</p>
    #[serde(rename = "satelliteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_id: Option<String>,
}

/// <p>Information about endpoints.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDetails {
    /// <p>ARN to a role needed for connecting streams to your instances. </p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The security groups to attach to the elastic network interfaces.</p>
    #[serde(rename = "securityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>A list of subnets where AWS Ground Station places elastic network interfaces to send streams to your instances.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>Information about the socket address.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocketAddress {
    /// <p>Name of a socket address.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Port of a socket address.</p>
    #[serde(rename = "port")]
    pub port: i64,
}

/// <p>Object that describes a spectral <code>Config</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpectrumConfig {
    /// <p>Bandwidth of a spectral <code>Config</code>.</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: FrequencyBandwidth,
    /// <p>Center frequency of a spectral <code>Config</code>.</p>
    #[serde(rename = "centerFrequency")]
    pub center_frequency: Frequency,
    /// <p>Polarization of a spectral <code>Config</code>.</p>
    #[serde(rename = "polarization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polarization: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>ARN of a resource tag.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>Tags assigned to a resource.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Object that determines whether tracking should be used during a contact executed with this <code>Config</code> in the mission profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackingConfig {
    /// <p>Current setting for autotrack.</p>
    #[serde(rename = "autotrack")]
    pub autotrack: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>ARN of a resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>Keys of a resource tag.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigRequest {
    /// <p>Parameters of a <code>Config</code>.</p>
    #[serde(rename = "configData")]
    pub config_data: ConfigTypeData,
    /// <p>UUID of a <code>Config</code>.</p>
    #[serde(rename = "configId")]
    pub config_id: String,
    /// <p>Type of a <code>Config</code>.</p>
    #[serde(rename = "configType")]
    pub config_type: String,
    /// <p>Name of a <code>Config</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMissionProfileRequest {
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "contactPostPassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_post_pass_duration_seconds: Option<i64>,
    /// <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    #[serde(rename = "contactPrePassDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_pre_pass_duration_seconds: Option<i64>,
    /// <p>A list of lists of ARNs. Each list of ARNs is an edge, with a <i>from</i> <code>Config</code> and a <i>to</i> <code>Config</code>.</p>
    #[serde(rename = "dataflowEdges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataflow_edges: Option<Vec<Vec<String>>>,
    /// <p>Smallest amount of time in seconds that you’d like to see for an available contact. AWS Ground Station will not present you with contacts shorter than this duration.</p>
    #[serde(rename = "minimumViableContactDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_viable_contact_duration_seconds: Option<i64>,
    /// <p>UUID of a mission profile.</p>
    #[serde(rename = "missionProfileId")]
    pub mission_profile_id: String,
    /// <p>Name of a mission profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>ARN of a tracking <code>Config</code>.</p>
    #[serde(rename = "trackingConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_config_arn: Option<String>,
}

/// <p>Information about an uplink echo <code>Config</code>.</p> <p>Parameters from the <code>AntennaUplinkConfig</code>, corresponding to the specified <code>AntennaUplinkConfigArn</code>, are used when this <code>UplinkEchoConfig</code> is used in a contact.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UplinkEchoConfig {
    /// <p>ARN of an uplink <code>Config</code>.</p>
    #[serde(rename = "antennaUplinkConfigArn")]
    pub antenna_uplink_config_arn: String,
    /// <p>Whether or not an uplink <code>Config</code> is enabled.</p>
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

/// <p>Information about the uplink spectral <code>Config</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UplinkSpectrumConfig {
    /// <p>Center frequency of an uplink spectral <code>Config</code>.</p>
    #[serde(rename = "centerFrequency")]
    pub center_frequency: Frequency,
    /// <p>Polarization of an uplink spectral <code>Config</code>.</p>
    #[serde(rename = "polarization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polarization: Option<String>,
}

/// Errors returned by CancelContact
#[derive(Debug, PartialEq)]
pub enum CancelContactError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl CancelContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(CancelContactError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CancelContactError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelContactError::Dependency(ref cause) => write!(f, "{}", cause),
            CancelContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CancelContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelContactError {}
/// Errors returned by CreateConfig
#[derive(Debug, PartialEq)]
pub enum CreateConfigError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Account limits for this resource have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(CreateConfigError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateConfigError::InvalidParameter(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateConfigError::ResourceLimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateConfigError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigError::Dependency(ref cause) => write!(f, "{}", cause),
            CreateConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateConfigError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigError {}
/// Errors returned by CreateDataflowEndpointGroup
#[derive(Debug, PartialEq)]
pub enum CreateDataflowEndpointGroupError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateDataflowEndpointGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDataflowEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(CreateDataflowEndpointGroupError::Dependency(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        CreateDataflowEndpointGroupError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateDataflowEndpointGroupError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDataflowEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDataflowEndpointGroupError::Dependency(ref cause) => write!(f, "{}", cause),
            CreateDataflowEndpointGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateDataflowEndpointGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDataflowEndpointGroupError {}
/// Errors returned by CreateMissionProfile
#[derive(Debug, PartialEq)]
pub enum CreateMissionProfileError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateMissionProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMissionProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(CreateMissionProfileError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateMissionProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateMissionProfileError::ResourceNotFound(
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
impl fmt::Display for CreateMissionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMissionProfileError::Dependency(ref cause) => write!(f, "{}", cause),
            CreateMissionProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateMissionProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMissionProfileError {}
/// Errors returned by DeleteConfig
#[derive(Debug, PartialEq)]
pub enum DeleteConfigError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(DeleteConfigError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteConfigError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteConfigError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigError::Dependency(ref cause) => write!(f, "{}", cause),
            DeleteConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigError {}
/// Errors returned by DeleteDataflowEndpointGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDataflowEndpointGroupError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteDataflowEndpointGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDataflowEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(DeleteDataflowEndpointGroupError::Dependency(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteDataflowEndpointGroupError::InvalidParameter(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteDataflowEndpointGroupError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDataflowEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDataflowEndpointGroupError::Dependency(ref cause) => write!(f, "{}", cause),
            DeleteDataflowEndpointGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteDataflowEndpointGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDataflowEndpointGroupError {}
/// Errors returned by DeleteMissionProfile
#[derive(Debug, PartialEq)]
pub enum DeleteMissionProfileError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteMissionProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMissionProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(DeleteMissionProfileError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteMissionProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteMissionProfileError::ResourceNotFound(
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
impl fmt::Display for DeleteMissionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMissionProfileError::Dependency(ref cause) => write!(f, "{}", cause),
            DeleteMissionProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteMissionProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMissionProfileError {}
/// Errors returned by DescribeContact
#[derive(Debug, PartialEq)]
pub enum DescribeContactError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(DescribeContactError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeContactError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeContactError::Dependency(ref cause) => write!(f, "{}", cause),
            DescribeContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeContactError {}
/// Errors returned by GetConfig
#[derive(Debug, PartialEq)]
pub enum GetConfigError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl GetConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(GetConfigError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetConfigError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetConfigError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigError::Dependency(ref cause) => write!(f, "{}", cause),
            GetConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConfigError {}
/// Errors returned by GetDataflowEndpointGroup
#[derive(Debug, PartialEq)]
pub enum GetDataflowEndpointGroupError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl GetDataflowEndpointGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataflowEndpointGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(GetDataflowEndpointGroupError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDataflowEndpointGroupError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDataflowEndpointGroupError::ResourceNotFound(
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
impl fmt::Display for GetDataflowEndpointGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDataflowEndpointGroupError::Dependency(ref cause) => write!(f, "{}", cause),
            GetDataflowEndpointGroupError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDataflowEndpointGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDataflowEndpointGroupError {}
/// Errors returned by GetMinuteUsage
#[derive(Debug, PartialEq)]
pub enum GetMinuteUsageError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl GetMinuteUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMinuteUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(GetMinuteUsageError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetMinuteUsageError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMinuteUsageError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMinuteUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMinuteUsageError::Dependency(ref cause) => write!(f, "{}", cause),
            GetMinuteUsageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetMinuteUsageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMinuteUsageError {}
/// Errors returned by GetMissionProfile
#[derive(Debug, PartialEq)]
pub enum GetMissionProfileError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl GetMissionProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMissionProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(GetMissionProfileError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetMissionProfileError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetMissionProfileError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMissionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMissionProfileError::Dependency(ref cause) => write!(f, "{}", cause),
            GetMissionProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetMissionProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMissionProfileError {}
/// Errors returned by GetSatellite
#[derive(Debug, PartialEq)]
pub enum GetSatelliteError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl GetSatelliteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSatelliteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(GetSatelliteError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetSatelliteError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSatelliteError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSatelliteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSatelliteError::Dependency(ref cause) => write!(f, "{}", cause),
            GetSatelliteError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetSatelliteError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSatelliteError {}
/// Errors returned by ListConfigs
#[derive(Debug, PartialEq)]
pub enum ListConfigsError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListConfigsError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListConfigsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListConfigsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigsError::Dependency(ref cause) => write!(f, "{}", cause),
            ListConfigsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListConfigsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigsError {}
/// Errors returned by ListContacts
#[derive(Debug, PartialEq)]
pub enum ListContactsError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListContactsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContactsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListContactsError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListContactsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListContactsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListContactsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListContactsError::Dependency(ref cause) => write!(f, "{}", cause),
            ListContactsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListContactsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListContactsError {}
/// Errors returned by ListDataflowEndpointGroups
#[derive(Debug, PartialEq)]
pub enum ListDataflowEndpointGroupsError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListDataflowEndpointGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDataflowEndpointGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListDataflowEndpointGroupsError::Dependency(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListDataflowEndpointGroupsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDataflowEndpointGroupsError::ResourceNotFound(
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
impl fmt::Display for ListDataflowEndpointGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDataflowEndpointGroupsError::Dependency(ref cause) => write!(f, "{}", cause),
            ListDataflowEndpointGroupsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListDataflowEndpointGroupsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDataflowEndpointGroupsError {}
/// Errors returned by ListGroundStations
#[derive(Debug, PartialEq)]
pub enum ListGroundStationsError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListGroundStationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGroundStationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListGroundStationsError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListGroundStationsError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListGroundStationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGroundStationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGroundStationsError::Dependency(ref cause) => write!(f, "{}", cause),
            ListGroundStationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListGroundStationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGroundStationsError {}
/// Errors returned by ListMissionProfiles
#[derive(Debug, PartialEq)]
pub enum ListMissionProfilesError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListMissionProfilesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMissionProfilesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListMissionProfilesError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListMissionProfilesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListMissionProfilesError::ResourceNotFound(
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
impl fmt::Display for ListMissionProfilesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMissionProfilesError::Dependency(ref cause) => write!(f, "{}", cause),
            ListMissionProfilesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListMissionProfilesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMissionProfilesError {}
/// Errors returned by ListSatellites
#[derive(Debug, PartialEq)]
pub enum ListSatellitesError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListSatellitesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSatellitesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListSatellitesError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListSatellitesError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSatellitesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSatellitesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSatellitesError::Dependency(ref cause) => write!(f, "{}", cause),
            ListSatellitesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListSatellitesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSatellitesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ListTagsForResourceError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::Dependency(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ReserveContact
#[derive(Debug, PartialEq)]
pub enum ReserveContactError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl ReserveContactError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReserveContactError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(ReserveContactError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ReserveContactError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ReserveContactError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReserveContactError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReserveContactError::Dependency(ref cause) => write!(f, "{}", cause),
            ReserveContactError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ReserveContactError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReserveContactError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(TagResourceError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::Dependency(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(UntagResourceError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::Dependency(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateConfig
#[derive(Debug, PartialEq)]
pub enum UpdateConfigError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(UpdateConfigError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateConfigError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateConfigError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigError::Dependency(ref cause) => write!(f, "{}", cause),
            UpdateConfigError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateConfigError {}
/// Errors returned by UpdateMissionProfile
#[derive(Debug, PartialEq)]
pub enum UpdateMissionProfileError {
    /// <p>Dependency encountered an error.</p>
    Dependency(String),
    /// <p>One or more parameters are not valid.</p>
    InvalidParameter(String),
    /// <p>Resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateMissionProfileError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMissionProfileError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "DependencyException" => {
                    return RusotoError::Service(UpdateMissionProfileError::Dependency(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateMissionProfileError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateMissionProfileError::ResourceNotFound(
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
impl fmt::Display for UpdateMissionProfileError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMissionProfileError::Dependency(ref cause) => write!(f, "{}", cause),
            UpdateMissionProfileError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateMissionProfileError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMissionProfileError {}
/// Trait representing the capabilities of the AWS Ground Station API. AWS Ground Station clients implement this trait.
#[async_trait]
pub trait GroundStation {
    /// <p>Cancels a contact with a specified contact ID.</p>
    async fn cancel_contact(
        &self,
        input: CancelContactRequest,
    ) -> Result<ContactIdResponse, RusotoError<CancelContactError>>;

    /// <p>Creates a <code>Config</code> with the specified <code>configData</code> parameters.</p> <p>Only one type of <code>configData</code> can be specified.</p>
    async fn create_config(
        &self,
        input: CreateConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<CreateConfigError>>;

    /// <p>Creates a <code>DataflowEndpoint</code> group containing the specified list of <code>DataflowEndpoint</code> objects.</p> <p>The <code>name</code> field in each endpoint is used in your mission profile <code>DataflowEndpointConfig</code> to specify which endpoints to use during a contact.</p> <p>When a contact uses multiple <code>DataflowEndpointConfig</code> objects, each <code>Config</code> must match a <code>DataflowEndpoint</code> in the same group.</p>
    async fn create_dataflow_endpoint_group(
        &self,
        input: CreateDataflowEndpointGroupRequest,
    ) -> Result<DataflowEndpointGroupIdResponse, RusotoError<CreateDataflowEndpointGroupError>>;

    /// <p>Creates a mission profile.</p> <p> <code>dataflowEdges</code> is a list of lists of strings. Each lower level list of strings has two elements: a <i>from</i> ARN and a <i>to</i> ARN.</p>
    async fn create_mission_profile(
        &self,
        input: CreateMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<CreateMissionProfileError>>;

    /// <p>Deletes a <code>Config</code>.</p>
    async fn delete_config(
        &self,
        input: DeleteConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<DeleteConfigError>>;

    /// <p>Deletes a dataflow endpoint group.</p>
    async fn delete_dataflow_endpoint_group(
        &self,
        input: DeleteDataflowEndpointGroupRequest,
    ) -> Result<DataflowEndpointGroupIdResponse, RusotoError<DeleteDataflowEndpointGroupError>>;

    /// <p>Deletes a mission profile.</p>
    async fn delete_mission_profile(
        &self,
        input: DeleteMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<DeleteMissionProfileError>>;

    /// <p>Describes an existing contact.</p>
    async fn describe_contact(
        &self,
        input: DescribeContactRequest,
    ) -> Result<DescribeContactResponse, RusotoError<DescribeContactError>>;

    /// <p>Returns <code>Config</code> information.</p> <p>Only one <code>Config</code> response can be returned.</p>
    async fn get_config(
        &self,
        input: GetConfigRequest,
    ) -> Result<GetConfigResponse, RusotoError<GetConfigError>>;

    /// <p>Returns the dataflow endpoint group.</p>
    async fn get_dataflow_endpoint_group(
        &self,
        input: GetDataflowEndpointGroupRequest,
    ) -> Result<GetDataflowEndpointGroupResponse, RusotoError<GetDataflowEndpointGroupError>>;

    /// <p>Returns the number of minutes used by account.</p>
    async fn get_minute_usage(
        &self,
        input: GetMinuteUsageRequest,
    ) -> Result<GetMinuteUsageResponse, RusotoError<GetMinuteUsageError>>;

    /// <p>Returns a mission profile.</p>
    async fn get_mission_profile(
        &self,
        input: GetMissionProfileRequest,
    ) -> Result<GetMissionProfileResponse, RusotoError<GetMissionProfileError>>;

    /// <p>Returns a satellite.</p>
    async fn get_satellite(
        &self,
        input: GetSatelliteRequest,
    ) -> Result<GetSatelliteResponse, RusotoError<GetSatelliteError>>;

    /// <p>Returns a list of <code>Config</code> objects.</p>
    async fn list_configs(
        &self,
        input: ListConfigsRequest,
    ) -> Result<ListConfigsResponse, RusotoError<ListConfigsError>>;

    /// <p>Returns a list of contacts.</p> <p>If <code>statusList</code> contains AVAILABLE, the request must include <code>groundStation</code>, <code>missionprofileArn</code>, and <code>satelliteArn</code>. </p>
    async fn list_contacts(
        &self,
        input: ListContactsRequest,
    ) -> Result<ListContactsResponse, RusotoError<ListContactsError>>;

    /// <p>Returns a list of <code>DataflowEndpoint</code> groups.</p>
    async fn list_dataflow_endpoint_groups(
        &self,
        input: ListDataflowEndpointGroupsRequest,
    ) -> Result<ListDataflowEndpointGroupsResponse, RusotoError<ListDataflowEndpointGroupsError>>;

    /// <p>Returns a list of ground stations. </p>
    async fn list_ground_stations(
        &self,
        input: ListGroundStationsRequest,
    ) -> Result<ListGroundStationsResponse, RusotoError<ListGroundStationsError>>;

    /// <p>Returns a list of mission profiles.</p>
    async fn list_mission_profiles(
        &self,
        input: ListMissionProfilesRequest,
    ) -> Result<ListMissionProfilesResponse, RusotoError<ListMissionProfilesError>>;

    /// <p>Returns a list of satellites.</p>
    async fn list_satellites(
        &self,
        input: ListSatellitesRequest,
    ) -> Result<ListSatellitesResponse, RusotoError<ListSatellitesError>>;

    /// <p>Returns a list of tags for a specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Reserves a contact using specified parameters.</p>
    async fn reserve_contact(
        &self,
        input: ReserveContactRequest,
    ) -> Result<ContactIdResponse, RusotoError<ReserveContactError>>;

    /// <p>Assigns a tag to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deassigns a resource tag.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the <code>Config</code> used when scheduling contacts.</p> <p>Updating a <code>Config</code> will not update the execution parameters for existing future contacts scheduled with this <code>Config</code>.</p>
    async fn update_config(
        &self,
        input: UpdateConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<UpdateConfigError>>;

    /// <p>Updates a mission profile.</p> <p>Updating a mission profile will not update the execution parameters for existing future contacts.</p>
    async fn update_mission_profile(
        &self,
        input: UpdateMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<UpdateMissionProfileError>>;
}
/// A client for the AWS Ground Station API.
#[derive(Clone)]
pub struct GroundStationClient {
    client: Client,
    region: region::Region,
}

impl GroundStationClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GroundStationClient {
        GroundStationClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GroundStationClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GroundStationClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GroundStationClient {
        GroundStationClient { client, region }
    }
}

#[async_trait]
impl GroundStation for GroundStationClient {
    /// <p>Cancels a contact with a specified contact ID.</p>
    async fn cancel_contact(
        &self,
        input: CancelContactRequest,
    ) -> Result<ContactIdResponse, RusotoError<CancelContactError>> {
        let request_uri = format!("/contact/{contact_id}", contact_id = input.contact_id);

        let mut request = SignedRequest::new("DELETE", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ContactIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelContactError::from_response(response))
        }
    }

    /// <p>Creates a <code>Config</code> with the specified <code>configData</code> parameters.</p> <p>Only one type of <code>configData</code> can be specified.</p>
    async fn create_config(
        &self,
        input: CreateConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<CreateConfigError>> {
        let request_uri = "/config";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<ConfigIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigError::from_response(response))
        }
    }

    /// <p>Creates a <code>DataflowEndpoint</code> group containing the specified list of <code>DataflowEndpoint</code> objects.</p> <p>The <code>name</code> field in each endpoint is used in your mission profile <code>DataflowEndpointConfig</code> to specify which endpoints to use during a contact.</p> <p>When a contact uses multiple <code>DataflowEndpointConfig</code> objects, each <code>Config</code> must match a <code>DataflowEndpoint</code> in the same group.</p>
    async fn create_dataflow_endpoint_group(
        &self,
        input: CreateDataflowEndpointGroupRequest,
    ) -> Result<DataflowEndpointGroupIdResponse, RusotoError<CreateDataflowEndpointGroupError>>
    {
        let request_uri = "/dataflowEndpointGroup";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<DataflowEndpointGroupIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDataflowEndpointGroupError::from_response(response))
        }
    }

    /// <p>Creates a mission profile.</p> <p> <code>dataflowEdges</code> is a list of lists of strings. Each lower level list of strings has two elements: a <i>from</i> ARN and a <i>to</i> ARN.</p>
    async fn create_mission_profile(
        &self,
        input: CreateMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<CreateMissionProfileError>> {
        let request_uri = "/missionprofile";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<MissionProfileIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMissionProfileError::from_response(response))
        }
    }

    /// <p>Deletes a <code>Config</code>.</p>
    async fn delete_config(
        &self,
        input: DeleteConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<DeleteConfigError>> {
        let request_uri = format!(
            "/config/{config_type}/{config_id}",
            config_id = input.config_id,
            config_type = input.config_type
        );

        let mut request = SignedRequest::new("DELETE", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ConfigIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigError::from_response(response))
        }
    }

    /// <p>Deletes a dataflow endpoint group.</p>
    async fn delete_dataflow_endpoint_group(
        &self,
        input: DeleteDataflowEndpointGroupRequest,
    ) -> Result<DataflowEndpointGroupIdResponse, RusotoError<DeleteDataflowEndpointGroupError>>
    {
        let request_uri = format!(
            "/dataflowEndpointGroup/{dataflow_endpoint_group_id}",
            dataflow_endpoint_group_id = input.dataflow_endpoint_group_id
        );

        let mut request = SignedRequest::new("DELETE", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DataflowEndpointGroupIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDataflowEndpointGroupError::from_response(response))
        }
    }

    /// <p>Deletes a mission profile.</p>
    async fn delete_mission_profile(
        &self,
        input: DeleteMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<DeleteMissionProfileError>> {
        let request_uri = format!(
            "/missionprofile/{mission_profile_id}",
            mission_profile_id = input.mission_profile_id
        );

        let mut request = SignedRequest::new("DELETE", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<MissionProfileIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMissionProfileError::from_response(response))
        }
    }

    /// <p>Describes an existing contact.</p>
    async fn describe_contact(
        &self,
        input: DescribeContactRequest,
    ) -> Result<DescribeContactResponse, RusotoError<DescribeContactError>> {
        let request_uri = format!("/contact/{contact_id}", contact_id = input.contact_id);

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeContactResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeContactError::from_response(response))
        }
    }

    /// <p>Returns <code>Config</code> information.</p> <p>Only one <code>Config</code> response can be returned.</p>
    async fn get_config(
        &self,
        input: GetConfigRequest,
    ) -> Result<GetConfigResponse, RusotoError<GetConfigError>> {
        let request_uri = format!(
            "/config/{config_type}/{config_id}",
            config_id = input.config_id,
            config_type = input.config_type
        );

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigError::from_response(response))
        }
    }

    /// <p>Returns the dataflow endpoint group.</p>
    async fn get_dataflow_endpoint_group(
        &self,
        input: GetDataflowEndpointGroupRequest,
    ) -> Result<GetDataflowEndpointGroupResponse, RusotoError<GetDataflowEndpointGroupError>> {
        let request_uri = format!(
            "/dataflowEndpointGroup/{dataflow_endpoint_group_id}",
            dataflow_endpoint_group_id = input.dataflow_endpoint_group_id
        );

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDataflowEndpointGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDataflowEndpointGroupError::from_response(response))
        }
    }

    /// <p>Returns the number of minutes used by account.</p>
    async fn get_minute_usage(
        &self,
        input: GetMinuteUsageRequest,
    ) -> Result<GetMinuteUsageResponse, RusotoError<GetMinuteUsageError>> {
        let request_uri = "/minute-usage";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<GetMinuteUsageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMinuteUsageError::from_response(response))
        }
    }

    /// <p>Returns a mission profile.</p>
    async fn get_mission_profile(
        &self,
        input: GetMissionProfileRequest,
    ) -> Result<GetMissionProfileResponse, RusotoError<GetMissionProfileError>> {
        let request_uri = format!(
            "/missionprofile/{mission_profile_id}",
            mission_profile_id = input.mission_profile_id
        );

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetMissionProfileResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetMissionProfileError::from_response(response))
        }
    }

    /// <p>Returns a satellite.</p>
    async fn get_satellite(
        &self,
        input: GetSatelliteRequest,
    ) -> Result<GetSatelliteResponse, RusotoError<GetSatelliteError>> {
        let request_uri = format!(
            "/satellite/{satellite_id}",
            satellite_id = input.satellite_id
        );

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetSatelliteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSatelliteError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>Config</code> objects.</p>
    async fn list_configs(
        &self,
        input: ListConfigsRequest,
    ) -> Result<ListConfigsResponse, RusotoError<ListConfigsError>> {
        let request_uri = "/config";

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigsError::from_response(response))
        }
    }

    /// <p>Returns a list of contacts.</p> <p>If <code>statusList</code> contains AVAILABLE, the request must include <code>groundStation</code>, <code>missionprofileArn</code>, and <code>satelliteArn</code>. </p>
    async fn list_contacts(
        &self,
        input: ListContactsRequest,
    ) -> Result<ListContactsResponse, RusotoError<ListContactsError>> {
        let request_uri = "/contacts";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<ListContactsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListContactsError::from_response(response))
        }
    }

    /// <p>Returns a list of <code>DataflowEndpoint</code> groups.</p>
    async fn list_dataflow_endpoint_groups(
        &self,
        input: ListDataflowEndpointGroupsRequest,
    ) -> Result<ListDataflowEndpointGroupsResponse, RusotoError<ListDataflowEndpointGroupsError>>
    {
        let request_uri = "/dataflowEndpointGroup";

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListDataflowEndpointGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDataflowEndpointGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of ground stations. </p>
    async fn list_ground_stations(
        &self,
        input: ListGroundStationsRequest,
    ) -> Result<ListGroundStationsResponse, RusotoError<ListGroundStationsError>> {
        let request_uri = "/groundstation";

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.satellite_id {
            params.put("satelliteId", x);
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
                .deserialize::<ListGroundStationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListGroundStationsError::from_response(response))
        }
    }

    /// <p>Returns a list of mission profiles.</p>
    async fn list_mission_profiles(
        &self,
        input: ListMissionProfilesRequest,
    ) -> Result<ListMissionProfilesResponse, RusotoError<ListMissionProfilesError>> {
        let request_uri = "/missionprofile";

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListMissionProfilesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMissionProfilesError::from_response(response))
        }
    }

    /// <p>Returns a list of satellites.</p>
    async fn list_satellites(
        &self,
        input: ListSatellitesRequest,
    ) -> Result<ListSatellitesResponse, RusotoError<ListSatellitesError>> {
        let request_uri = "/satellite";

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<ListSatellitesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSatellitesError::from_response(response))
        }
    }

    /// <p>Returns a list of tags for a specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "groundstation", &self.region, &request_uri);
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

    /// <p>Reserves a contact using specified parameters.</p>
    async fn reserve_contact(
        &self,
        input: ReserveContactRequest,
    ) -> Result<ContactIdResponse, RusotoError<ReserveContactError>> {
        let request_uri = "/contact";

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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
                .deserialize::<ContactIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ReserveContactError::from_response(response))
        }
    }

    /// <p>Assigns a tag to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "groundstation", &self.region, &request_uri);
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

    /// <p>Deassigns a resource tag.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "groundstation", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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

    /// <p>Updates the <code>Config</code> used when scheduling contacts.</p> <p>Updating a <code>Config</code> will not update the execution parameters for existing future contacts scheduled with this <code>Config</code>.</p>
    async fn update_config(
        &self,
        input: UpdateConfigRequest,
    ) -> Result<ConfigIdResponse, RusotoError<UpdateConfigError>> {
        let request_uri = format!(
            "/config/{config_type}/{config_id}",
            config_id = input.config_id,
            config_type = input.config_type
        );

        let mut request = SignedRequest::new("PUT", "groundstation", &self.region, &request_uri);
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
                .deserialize::<ConfigIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConfigError::from_response(response))
        }
    }

    /// <p>Updates a mission profile.</p> <p>Updating a mission profile will not update the execution parameters for existing future contacts.</p>
    async fn update_mission_profile(
        &self,
        input: UpdateMissionProfileRequest,
    ) -> Result<MissionProfileIdResponse, RusotoError<UpdateMissionProfileError>> {
        let request_uri = format!(
            "/missionprofile/{mission_profile_id}",
            mission_profile_id = input.mission_profile_id
        );

        let mut request = SignedRequest::new("PUT", "groundstation", &self.region, &request_uri);
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
                .deserialize::<MissionProfileIdResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMissionProfileError::from_response(response))
        }
    }
}
