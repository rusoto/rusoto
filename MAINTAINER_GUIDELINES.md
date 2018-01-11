## Maintainer Guidelines

### Pull request review

1. New service has integration test(s)
2. All integration tests pass
3. Code looks sound
4. Travis/Appveyor builds pass
5. Rustfmt has been run

### Cutting a release

1. Make a new branch for the release, following the [RELEASING](RELEASING.md) document. ([Example release PR](https://github.com/rusoto/rusoto/pull/883))
2. Run all integration tests on the PR branch.
3. After release PR is reviewed and merged, follow instructions in [RELEASING](RELEASING.md).

### Development direction

* Rusoto should be able to be used and developed on Rust's stable channel
* Rusoto's 1.0 release should have all known AWS related bugs fixed (Ceph and Minio are not required for 1.0) and all services implemented and covered with integration tests (if possible)
