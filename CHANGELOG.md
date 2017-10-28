# Rusoto changes

## [Unreleased]

(Please put an entry here in each PR)
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
- Added Pinpoint service

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
