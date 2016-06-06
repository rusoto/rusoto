#![cfg(all(feature = "ets", feature = "s3"))]

extern crate env_logger;
#[macro_use] extern crate log;
extern crate rand;
extern crate rusoto;

use std::clone::Clone;
use std::ops::{Deref, DerefMut};

use rand::Rng;
use rusoto::{ChainProvider, ProvideAwsCredentials, Region};
use rusoto::ets::EtsClient;
use rusoto::s3::{BucketName, S3Helper};

const AWS_ETS_WEB_PRESET_ID: &'static str = "1351620000001-100070";
const AWS_ETS_WEB_PRESET_NAME: &'static str = "System preset: Web";
const AWS_REGION: Region = Region::UsEast1;
const AWS_SERVICE_RANDOM_SUFFIX_LENGTH: usize = 20;

struct TestEtsClient<P>
    where P: ProvideAwsCredentials
{
    credentials_provider: P,
    region: Region,

    client: EtsClient<P>,

    s3_helper: Option<S3Helper<P>>,
    input_bucket: Option<BucketName>,
    output_bucket: Option<BucketName>,
}

impl<P> TestEtsClient<P>
    where P: ProvideAwsCredentials + Clone
{
    /// Creates a new `EtsClient` for a test.
    fn new(credentials_provider: P, region: Region) -> Self {
        TestEtsClient {
            credentials_provider: credentials_provider.clone(),
            region: region,
            client: EtsClient::new(credentials_provider, region),
            s3_helper: None,
            input_bucket: None,
            output_bucket: None,
        }
    }

    fn create_s3_helper(&mut self) {
        self.s3_helper = Some(S3Helper::new(
            self.credentials_provider.clone(),
            self.region
        ));
    }

    fn create_bucket(&mut self) -> BucketName {
        let bucket_name = generate_unique_name("ets-bucket-1");
        let result = self.s3_helper
            .as_mut()
            .unwrap()
            .create_bucket_in_region(&bucket_name, AWS_REGION, None);
        let mut location = result
            .unwrap()
            .location;
        // A `Location` is identical to a `BucketName` except that it has a
        // forward slash prepended to it, so we need to remove it.
        location.remove(0);
        info!("Created S3 bucket: {}", location);
        location
    }
}

impl<P> Deref for TestEtsClient<P>
    where P: ProvideAwsCredentials
{
    type Target = EtsClient<P>;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl<P> DerefMut for TestEtsClient<P>
    where P: ProvideAwsCredentials
{
    fn deref_mut<'a>(&'a mut self) -> &'a mut EtsClient<P> {
        &mut self.client
    }
}

impl<P> Drop for TestEtsClient<P>
    where P: ProvideAwsCredentials
{
    fn drop(&mut self) {
        self.s3_helper.take().map(|s3_helper| {
            self.input_bucket.take().map(|bucket| {
                match s3_helper.delete_bucket(&bucket, self.region) {
                    Ok(_) => { info!("Deleted S3 bucket: {}", bucket) },
                    Err(e) => { error!("Failed to delete S3 bucket: {}", e) }
                };
            });
            self.output_bucket.take().map(|bucket| {
                match s3_helper.delete_bucket(&bucket, self.region) {
                    Ok(_) => { info!("Deleted S3 bucket: {}", bucket) },
                    Err(e) => { error!("Failed to delete S3 bucket: {}", e) }
                };
            });
        });
    }
}

// TODO: once Rust has proper support for testing frameworks, this code will
// need to be refactored so that it is only called once, instead of per test
// case.
fn initialize() {
    let _ = env_logger::init();
}

fn create_ets_client() -> TestEtsClient<ChainProvider> {
    TestEtsClient::new(
        ChainProvider::new().unwrap(),
        AWS_REGION
    )
}

/// Generates a random name for an AWS service by appending a random sequence of
/// ASCII characters to the specified prefix.
fn generate_unique_name(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        rand::thread_rng()
            .gen_ascii_chars()
            .take(AWS_SERVICE_RANDOM_SUFFIX_LENGTH)
            .collect::<String>()
    )
}

#[test]
#[should_panic(expected = "arn cannot be null")]
fn create_pipeline_without_arn() {
    use rusoto::ets::CreatePipelineRequest;

    initialize();

    let mut ets = create_ets_client();
    ets.create_s3_helper();
    ets.input_bucket = Some(ets.create_bucket());
    ets.output_bucket = Some(ets.create_bucket());

    let request = CreatePipelineRequest {
        input_bucket: ets.input_bucket.as_ref().cloned().unwrap(),
        output_bucket: ets.output_bucket.as_ref().cloned(),
        ..CreatePipelineRequest::default()
    };
    let response = ets.create_pipeline(&request);

    response.unwrap();
}

#[test]
fn create_preset() {
    use rusoto::ets::{
        AudioCodecOptions,
        AudioParameters,
        CreatePresetRequest,
        DeletePresetRequest,
    };

    initialize();

    let ets = create_ets_client();

    let name = generate_unique_name("ets-preset-1");
    let request = CreatePresetRequest {
        audio: Some(AudioParameters {
            channels: Some("2".to_owned()),
            codec: Some("flac".to_owned()),
            codec_options: Some(AudioCodecOptions {
                bit_depth: Some("24".to_owned()),
                ..AudioCodecOptions::default()
            }),
            sample_rate: Some("96000".to_owned()),
            ..AudioParameters::default()
        }),
        container: "flac".to_owned(),
        description: Some("This is an example FLAC preset".to_owned()),
        name: name.clone(),
        ..CreatePresetRequest::default()
    };
    let response = ets.create_preset(&request);

    assert!(response.is_ok());

    let response = response.unwrap();

    assert!(response.preset.is_some());

    let preset = response.preset.unwrap();

    assert_eq!(preset.container, Some("flac".to_owned()));
    assert_eq!(preset.description, Some("This is an example FLAC preset".to_owned()));
    assert_eq!(preset.name, Some(name));
    assert!(preset.id.is_some());

    let id = preset.id.unwrap();

    info!("Created preset with id: {:?}", &id);

    // Cleanup

    let request = DeletePresetRequest {
        id: id,
    };
    ets.delete_preset(&request).ok();
}

#[test]
fn delete_preset() {
    use rusoto::ets::{
        AudioCodecOptions,
        AudioParameters,
        CreatePresetRequest,
        DeletePresetRequest,
    };

    initialize();

    let ets = create_ets_client();

    let name = generate_unique_name("ets-preset-1");
    let request = CreatePresetRequest {
        audio: Some(AudioParameters {
            channels: Some("2".to_owned()),
            codec: Some("flac".to_owned()),
            codec_options: Some(AudioCodecOptions {
                bit_depth: Some("24".to_owned()),
                ..AudioCodecOptions::default()
            }),
            sample_rate: Some("96000".to_owned()),
            ..AudioParameters::default()
        }),
        container: "flac".to_owned(),
        description: Some("This is an example FLAC preset".to_owned()),
        name: name.clone(),
        ..CreatePresetRequest::default()
    };
    let response = ets.create_preset(&request).unwrap();
    let preset = response.preset.unwrap();
    let id = preset.id.unwrap();

    let request = DeletePresetRequest {
        id: id.clone(),
    };
    let response = ets.delete_preset(&request);

    assert!(response.is_ok());
    info!("Deleted preset with id: {:?}", &id);
}

#[test]
fn list_jobs_by_status() {
    use rusoto::ets::ListJobsByStatusRequest;

    initialize();

    let ets = create_ets_client();

    let status = "Submitted".to_owned();
    let request = ListJobsByStatusRequest {
        status: status.clone(),
        ..ListJobsByStatusRequest::default()
    };
    let response = ets.list_jobs_by_status(&request);

    assert!(response.is_ok());

    let response = response.unwrap();

    info!("Got list of jobs with status \"{}\": {:?}", &status, response.jobs);
}

#[test]
fn list_pipelines() {
    use rusoto::ets::ListPipelinesRequest;

    initialize();

    let ets = create_ets_client();

    let request = ListPipelinesRequest::default();
    let response = ets.list_pipelines(&request);

    assert!(response.is_ok());

    let response = response.unwrap();

    info!("Got list of pipelines: {:?}", response.pipelines);
}

#[test]
fn list_presets() {
    use rusoto::ets::ListPresetsRequest;

    initialize();

    let ets = create_ets_client();

    let request = ListPresetsRequest::default();
    let response = ets.list_presets(&request);

    assert!(response.is_ok());

    let response = response.unwrap();

    assert!(response.presets.is_some());

    let presets = response.presets.unwrap();
    info!("Got list of presets.");
    for preset in presets.iter() {
        info!("Preset: {:?}", preset.name);
    }

    let web_preset = presets.iter()
        .filter(|x| x.id == Some(AWS_ETS_WEB_PRESET_ID.to_owned()))
        .next();

    assert!(web_preset.is_some());

    let web_preset = web_preset.unwrap();

    assert_eq!(web_preset.id, Some(AWS_ETS_WEB_PRESET_ID.to_owned()));
    assert_eq!(web_preset.name, Some(AWS_ETS_WEB_PRESET_NAME.to_owned()));
}

#[test]
fn read_preset() {
    use rusoto::ets::ReadPresetRequest;

    initialize();

    let ets = create_ets_client();

    let request = ReadPresetRequest {
        id: AWS_ETS_WEB_PRESET_ID.to_owned(),
    };
    let response = ets.read_preset(&request);

    assert!(response.is_ok());

    let response = response.unwrap();

    assert!(response.preset.is_some());

    let preset = response.preset.unwrap();
    info!("Got preset: {:?}", preset.name);

    assert_eq!(preset.id, Some(AWS_ETS_WEB_PRESET_ID.to_owned()));
    assert_eq!(preset.name, Some(AWS_ETS_WEB_PRESET_NAME.to_owned()));
}
