## Maintainer Guidelines

### Releasing working code

* run a full suite of actual integration tests before accepting a PR
* releases should have a full integration test run before releasing to catch stragglers

### Development

* Rusoto should be able to be used and developed on Rust's stable channel
* to hit Rusoto's 1.0 release we should have all known AWS related bugs fixed (Ceph and minio are not required for 1.0) and all services covered
