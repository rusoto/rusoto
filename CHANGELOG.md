# Rusoto changes

## [Unreleased]
- Forward traits through wrappers like Rc.
- Upgrade hyper-rustls library
- Fix duplicated Content-Type header in SageMaker Runtime
- Switch from `try!` to `?` operator
- Remove unneeded muts in Glacier codegen
- Add Eu-North-1 Region
- Fix bug in SNS publish message action
- Mock can simulate communications errors

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
