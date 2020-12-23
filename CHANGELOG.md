# Rusoto changes

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

(Please put changes here)

- Display `rusoto_core::Client` in docs
- Fix unsoundness in `rusoto_mock::MultipleMockRequestDispatcher`
- Swap the unmaintained `dirs` crate for its replacement `dirs-next`
- Swap `pin-project` for the lighter weight `pin-project-lite`
- Update to `base64` 0.13
- Update to `hmac` 0.10
- Update to `hyper-rustls` 0.21.
- Disable `chrono`'s `oldtime` feature
- Remove dependency on `regex`
- Update to botocore 1.19.42
- Add ability to set local agent appended to the default User-Agent

## [0.45.0] - 2020-07-22

- Add event-stream protocol support (currently only for JSON APIs, used in `subscribe_to_shard` call in Kinesis)
- Extract common generated code into utility functions to improve compile times
- Allow creating a ProfileProvider with only the profile
- CDATA sections are now treated like strings
- Fix incorrect type definition for `rusoto_batch::JobDetail`
- Update to `hmac` 0.8 and `sha2` 0.9
- Added `Sync` bounds to `AsyncRead` and `Read` structures
- Update to botocore 1.17.20

## [0.44.0] - 2020-06-01

- Add support for af-south-1, Africa (Cape Town), and eu-south-1, Europe (Milan)
- Update to botocore 1.16.14
- Fix `Time::now()` and `OffsetDateTime::now()` deprecation warnings
- Fix minimum version of `time` crate
- Always encode `+` in query strings
- Added a Cognito credential provider
- Add `MultipleMockRequestDispatcher` to permit mocking multiple requests using the same client
- Fix `rusoto_sts::WebIdentityProvider::from_k8s_env` always requiring `AWS_ROLE_SESSION_NAME` env var which should be optional
- Added support to optionally define a session policy when using `rusoto_sts::WebIdentityProvider`
- Omit generating XML-deseralization code for actions without a response body
- Add `region_from_profile()` function to ProfileProvider
- Fix applying `Content-Encoding`
- Added `new_with_size()` function to ByteStream
- Add defualt help text to Makefile

## [0.43.0] - 2020-03-15

- Fix minimum version of hyper
- Fix `PrimitiveDateTime` deprecation error
- Update to dirs 2.0
- Bump base64 to 0.12 and hyper-rustls to 0.20
- Fix `serialize_structs` and `deserialize_structs`
- Fix JWT serialization in `WebIdentityProvider`
- Add ability to set local agent prepended to the default User-Agent
- Fix invalid signature for Route 53 `resource_record_sets` methods
- Improve `Display` impl for `RusotoError::Unknown`
- Fix hang in XML deserialization for flattened shapes
- Remove obsolete `RusotoFuture` and fix docs generation on nightly
- Fix `credential_process`, again
- Change non-China S3 domains to `s3.{region}.amazonaws.com`

## [0.43.0-beta.1] - 2020-02-07

- Move to `std::future::Future`, async/.await, and Tokio 0.2
- Update to botocore 1.14.9
- Add Discord invite link to README.md and CONTRIBUTING.md
- Remove unused import
- Fixed links in AWS-CREDENTIALS.md
- Fixed SNS API's attributes and value keyword
- Adding support for web identity provider, which enables IAM roles for Kubernetes service accounts.
- Add object-safe AwsCredentialsProvider trait as alternative to the existing generic `ProvideAwsCredentials` trait.
- Introduce `Secret` type to automatically zero-out memory use to stored secret credentials. So far, only used in the new web identity provider.
- Introduce `Variable` to abstract over certain credential provider input parameters.
- Encode request payload optionally with Gzip
- Add Debug trait to generated Clients
- Add `rusoto_ec2::filter!` macro
- Improve `InstanceMetadataProvider` to avoid cloning unnecessarily
- Remove deprecated `Error::description` implementations
- Add features `serialize_structs` and `deserialize_structs`
- Implement Clone on various Credential structs.
- Fix incorrect encoding of Session Token when pre-signing URLs
- Add IoT Secure Tunneling service
- Fix Directory Service integration tests
- Update to time 0.2.x

## [0.42.0] - 2019-11-18

- Use static initializer for AWS profile regex
- Add QLDB service
- Add QLDB Session service
- Update Skeptic tests for Rusoto v0.41
- Don't decode query string parameters before encoding it. Results in fixing the prefix and marker
params for s3 `list_objects` methods
- Add Textract service
- Update CloudDirectory API definition to `2017-01-11`
- Add SecurityHub service
- Add Transfer service
- Introducing `rusoto_signature`, a standalone crate for signing HTTP requests.
- Make static credentials into a credential provider
- Add anonymous credentials support
- Don't trim whitepsace when parsing xml payload. Fixes truncating of items with spaces in payloads
such as an S3 key returned in `list_objects_v2`
- Region deserialization format matches what Region serializers expect: https://github.com/rusoto/rusoto/pull/1544
- Fixed regression of `x-amz-content-sha256` header not being signed: https://github.com/rusoto/rusoto/pull/1545
- Allow `rustls` to be used in `rusoto_mock`: https://github.com/rusoto/rusoto/pull/1557
- Added opt-in ability for service objects to be serialized: https://github.com/rusoto/rusoto/pull/1560
- Avoid panicking in credential provider when parsing credentials file: https://github.com/rusoto/rusoto/pull/1573

## [0.41.1] - 2019-10-11

- Fixed regression in `rusoto_credential` session token handling: https://github.com/rusoto/rusoto/pull/1528

## [0.41.0] - 2019-10-07

- Add `HttpClient::from_builder`
- Upgrade to botocore from `1.12.163` to `1.12.230`
- The types `rusoto_events::{CloudWatchEvents,CloudWatchEventsClient}` were renamed to `rusoto_events::{EventBridge,EventBridgeClient}`
- Deserialize PostTextResponse correctly by allowing null values in the slots field
- Fix Profile Config Loading: should ignore comments with '=' chars
- Add App Mesh service
- Fix service_crategen to parse operations with multiple static params
- Refactor S3 integration tests - about a `#[test]` per behavior
- Add support for non signing clients
- Add EC2 Instance Connect service
- Allow deserialization of regions without an endpoint specified
- Add ApNortheast3 region
- Add MeSouth1 region
- Add x-amz-content-sha256 header to signed and canonical headers
- Added `Eq` and `Hash` implementations on `Region`
- Fixed parsing of Athena error messages
- Fix credential_process behavior when using the non-default profile
- Correctly read session tokens from credential_process

## [0.40.0] - 2019-06-28

- Only emit types used in service during crate generation
- Updated CloudFront to use latest API version: `2018-11-05`
- Only emit crate tests section when needed
- Fix bug with CodePipeline response not containing required fields from AWS
- Moved API documentation links to docs.rs
- Decode IAM policy documents automatically
- Removed `serde_json` crate from services where it was not required
- Exclude `test_resources` in cargo manifest
- upgrades botocore version to from `1.12.156` to `1.12.163`
- (Breaking Change) Fix invalid signatures on presigned URLs by adding a new
  `should_sha256_sign_payload` argument to `SignedRequest::generate_presigned_url`.

## [0.39.0] - 2019-05-19

- Add Worklink service
- Add FSX service
- Fix de/serialization of DynamoDB binary set attribute values
- Change type for blob values from `Vec<u8>` to `Bytes`
- Add DocDB service
- Add License Manager service
- Add Kafka service
- Add Chime service
- Add RDS Data service
- Add ComprehendMedical service
- Add Ap-East-1 Region
- Remove log crate dependency from services
- Remove decoding of the uri path before encoding it
- Use http::HeaderMap instead of our custom implementation
- Update all public crates to Rust 2018 edition

## [0.38.0] - 2019-04-17

- Add `RusotoError` enum as base error type for all services
- Improve error messages for BufferedHttpResponse in Unknown error variants.
- Fix hostname derivation for custom Region endpoints
- Support presigned URLs for multipart uploads to S3
- Add Us-Gov-East region
- Fix a bug in SNS CreateTopic and Subscribe
- Reduced generated xml deserializer logic
- Move credentials crate to Rust 2018
- Remove internal test files from published crates for rusoto_credential and rusoto_core

## [0.37.0] - 2019-03-12

- Implement Clone on the various Client types.
- Upgrade hyper-rustls library
- Fix duplicated Content-Type header in SageMaker Runtime
- Switch from `try!` to `?` operator
- Remove unneeded muts in Glacier codegen
- Add Eu-North-1 Region
- Fix bug in SNS publish message action
- Mock can simulate communications errors
- Upgrade botocore definitions to [`1.12.100`](https://github.com/boto/botocore/tree/1.12.100)
- add [amplify](https://docs.aws.amazon.com/cli/latest/reference/amplify/index.html) service
- add [apigatewaymanagementapi](https://docs.aws.amazon.com/cli/latest/reference/apigatewaymanagementapi/index.html) service
- add [apigatewayv2](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/index.html) service
- add [ram](https://docs.aws.amazon.com/cli/latest/reference/ram/index.html) service
- Add [`credential_process`](https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes) support in `~/.aws/config`
- Add Route53 TXT record quoting helper
- Fix a malformed SNS Publish API request when it has message attributes

## [0.36.0] - 2018-12-04

- Add Connect service
- Add MediaTailor support
- Add ByteStream struct to core
- Skip serializing blobs when they are `Option::None`
- Fix typo in `service_crategen` README.md
- Add Kinesis Video Archived Media service
- Update regex to version 1
- Add Appsync service
- Handle s3 out of order elements
- Add mediaconvert service
- Add KinesisVideo support
- Add a filter that will limit what services to generate
- Enable passthrough parsing of payload members
- Add sagemaker-runtime
- Fix some glacier bugs
- Add CloudFront unit test
- Add IoT Data service
- Add MediaLive service
- Add ResourceGroups service
- Add Mobile service
- Skip serializing blobs when they are Option::None (DynamoDB)
- Fix import/export endpoint
- Add MediaPackage service
- Add IoT Jobs Data
- Add Kinesis Video Media service
- Add IoT Analytics
- Add IoT 1click devices
- Add Workmail service
- Add IoT 1Click Projects

## [0.35.0] - 2018-10-31

- Add Cost Explorer
- Add Performance Insights support
- Add ServiceDiscovery support
- Add Sentiment support
- Add Sagemaker support
- Add Transcribe service
- Added Neptune support
- Add GuardDuty service
- Add AWS Macie
- Adds EKS
- Add AWS Pricing service
- Add Translate service
- Add Firewall Management Service (FMS)
- Add Cloud9 support
- Add Autoscaling Plans
- Add MQ service
- Add `From<Result<T, E>> for RusotoFuture<T, E>` implementation for mocking.
- Add ACM PCA support
- rusoto_credential uses Serde derives for credentials instead of hand written code
- Add MediaStore support
- Expose raw BufferedHttpResponse on ::Unknown error variants
- Removed Ceph test for `Luminous`
- Honor profile region in `Default` implementation of `Region`
- Fix bug that could not authenticate ARN with colon
- Fix error parsing for services using boto's `rest-json` protocol published prior to this release . The following service crates were affected.
  * `apigateway`, `batch`, `clouddirectory`, `cloudsearchdomain`, `cognito-sync`,
  `efs`, `eks`, `elastictranscoder`, `glacier`, `greengrass`, `guardduty`, `iot`,
  `lambda`, `lex-models`, `lex-runtime`, `mq`, `polly`, `serverlessrepo`,
  `workdocs`, `xray`

## [0.34.0] - 2018-09-05

- Add example to Rusoto Logs documentation
- Add custom dev dependency capability to services crategen
- Allow replacing OpenSSL with rustls by adding `features = ["rustls"], default_features=false` to your Cargo.toml
- Fix codegen for query types not sending values correctly
- Bump minimum supported version of Rust to 1.26
- Make STS provider `Send`
- Remove unused package `hyper-tls` in credentials crate
- Send parameters in request body instead of query string for query based services and EC2
- Allow AWS credentials in environment variables to have a custom prefix
- Fix bug in presigned URLs for S3

## [0.33.1] - 2018-08-07

- Fix `rusoto_mock` versions available

## [0.33.0] - 2018-07-31

- Remove `impl Display for Region` since it was of little use and confusingly similar to `Region::name()`.
- More efficiently and correctly remove scheme from `Region::Custom` endpoints
- Prevent reactor from hanging indefinitely when using the new tokio release
- Fix deserialization for empty JSON responses
- Fixed bug in query services where lists had incorrect parent item in request
- Improve deserializer of XML error responses
- Adds Serverless Repo service
- Add Alexa for Business service
- Add [Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html) service
- Support streaming uploads for services like S3
- Implement `DispatchSignedRequest` and `ProvideAwsCredentials` for `Arc<>` and `Rc<>` wrapped types
- Changed HttpClient to be generic over hyper::Connect, default HttpClient<C = HttpsConnector<HttpConnector>>, see #1033
- Derive PartialEq for all request and response types, except when they contain streams
- Change core and credentials to use Hyper 0.12
- Add support for alternative prefixes for environment variables.

## [0.32.0] - 2018-03-03

- Convert all services to `futures`-based APIs
- Show secret keys and tokens as `"**********"` in `Debug` output
- Ensure list of signed headers is correct when not all headers are signed
- Use ```$AWS_PROFILE``` to obtain default profile name
- Implement `Default` for `Region`
- Derive Clone for remaining types (affects CloudFront, Route 53 and S3)
- Link to service-specific documentation in generated Cargo manifests
- Change credential expiration for non-temporary credentials to be optional and add support for ```AWS_CREDENTIAL_EXPIRATION``` to EnvironmentProvider
- Improve ContainerProvider to mimic the behavior of the other SDKs by also considering ```AWS_CONTAINER_AUTHORIZATION_TOKEN``` and ```AWS_CONTAINER_CREDENTIALS_FULL_URI```
- Implement per-call timeouts for the `DispatchSignedRequest` trait
- Implement timeouts for `ContainerProvider` and `InstanceMetadataProvider`

## [0.31.0] - 2018-01-21

- Update Rusoto Core ReadMe
- use correct China-northwest region domain
- Fix handling of error responses from Ceph (S3)
- Added integration tests for Ceph and Minio
- Convert metadata keys to lowercase (only affects third party services, Amazon already converts them)
- Allow setting both Region name and endpoint via `Region::Custom`
- Added China-northwest, US-Gov-West & Paris regions
- Switched crategen from rustfmt to rustfmt-nightly
- Removed unused AsciiExt imports
- S3 StreamingBody now has public constructor

## [0.30.0] - 2017-12-02

- Added CloudHSMv2
- Added ResourceGroupsTaggingApi
- Added Lex runtime
- Added Lex Models service
- S3 StreamingBody now implements Read trait
- Added Budgets service
- Send metadata fields to S3

## [0.29.0] - 2017-11-02

- Added CHANGELOG
- Updated CONTRIBUTING to explain PR process
- Added Application Autoscaling service
- Added Athena service
- Added X-Ray service
- Updated Credentials crate to use hyper 0.11 (aka the Async IO Update).
- Added Documentation to Credentials Crate.
- Make Rusoto Core use HTTP Pools to re-use connections.
- Fixed Edge Cases in URI Encoding of Rusoto (double query encoding, +'s in query strings).
- Updated ring dependency
- Added Cloud Directory service
- Added CloudSearch Domain service
- Added GreenGrass service
- Added Elastic Filesystem service
- Fix broken links on the STS documentation
- Use xml::EventWriter to serialize XML payload
- Added Workdocs service
- Added Cognito Sync service
- Added Shield service
- Added Glue service
- Added DynamoDB Accelerator
- Added Discovery service
- Added CodeStar service
- Added Migration Hub service
- Added Marketplace Entitlement service

## [0.28.0] - 2017-08-25

### Added
- Credentials: accept `aws_security_token` for backwards compatibility
- Codegen: add `check` command for missing or outdated services
- API Gateway support
- Mechanical Turk support
- Polly support
- Glacier support
- Header on files that are generated to guide changes the code generation
- AWS Batch support
- Use botocore provided documentation in our crate documentation
- Credentials crate allows unrecognized fields in credentials profile
- Route53 now sends request to the right endpoint
- Route53 integration test
- Streaming download support for S3
- Custom region now supported: used for local DynamoDB and API compatible services such as Minio and Ceph
- Code of Condcut

### Changed
- Moved root Cargo.toml to root of git project to allow git dependency references
- Updated botocore to 1.5.75
- Integration tests now build, but don't run, as part of the CI process
- Credentials crate got dependency upgrades
- REST protocols now sends requests with headers and bodies

### Removed
- Credentials crate no longer retries credential acquiring
- Type aliases removed.  Example: we no longer use `BucketName` which was an alias for `String`.
- travis-cargo from TravisCI builds

[Unreleased]: https://github.com/rusoto/rusoto/compare/rusoto-v0.45.0...HEAD
[0.45.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.44.0...rusoto-v0.45.0
[0.44.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.43.0...rusoto-v0.44.0
[0.43.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.43.0-beta.1...rusoto-v0.43.0
[0.43.0-beta.1]: https://github.com/rusoto/rusoto/compare/rusoto-v0.42.0...rusoto-v0.43.0-beta.1
[0.42.0]: https://github.com/rusoto/rusoto/compare/credentials-v0.41.1...rusoto-v0.42.0
[0.41.1]: https://github.com/rusoto/rusoto/compare/rusoto-v0.41.0...credentials-v0.41.1
[0.41.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.40.0...rusoto-v0.41.0
[0.40.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.39.0...rusoto-v0.40.0
[0.39.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.38.0...rusoto-v0.39.0
[0.38.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.37.0...rusoto-v0.38.0
[0.37.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.36.0...rusoto-v0.37.0
[0.36.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.35.0...rusoto-v0.36.0
[0.35.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.34.0...rusoto-v0.35.0
[0.34.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.33.1...rusoto-v0.34.0
[0.33.1]: https://github.com/rusoto/rusoto/compare/rusoto-v0.33.0...rusoto-v0.33.1
[0.33.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.32.0...rusoto-v0.33.0
[0.32.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.31.0...rusoto-v0.32.0
[0.31.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.30.0...rusoto-v0.31.0
[0.30.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.29.0...rusoto-v0.30.0
[0.29.0]: https://github.com/rusoto/rusoto/compare/rusoto-v0.28.0...rusoto-v0.29.0
[0.28.0]: https://github.com/rusoto/rusoto/releases/tag/rusoto-v0.28.0
