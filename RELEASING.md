## Guidelines

1. Rusoto [has a CHANGELOG](CHANGELOG.md) to track what's in releases.
2. Releases are tracked [in GitHub](https://github.com/rusoto/rusoto/releases).

Before hitting 1.0, the public API has no backwards compatibility guaranteed.

After 1.0, the public API will be stable for the 1.0 releases. If we need to break the public API, Rusoto 2.0 will be started. Patches will be back ported to the 1.x branch.

## Release trains

For pre-1.0.0:

* Targeting one release a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Publishing walkthrough:

1. Ensure all PRs included in the release are reflected in [the CHANGELOG](https://github.com/rusoto/rusoto/blob/master/CHANGELOG.md). If in doubt, add an entry so it's recorded. Can be a separate PR or part of the one below.
2. Make a pull request that bumps version numbers for `rusoto_core`, `rusoto_credential`, `rusoto_signature` and `rusoto_mock` if needed and each service that changed since previous release.  Service versions are in the `services.json` file in the codegen project. Otherwise they are in the `Cargo.toml` files for each project.  Make sure the root Rusoto README example gets updated with the new version. The `skeptical` package will also need to be set to use the new version of Rusoto - do this in a PR after the release and publish has been completed.
3. Merge release PR.
4. Publish new version of `rusoto_credential`.
5. Publish new version of `rusoto_signature`.
6. Publish new version of `rusoto_core`.
7. Publish new version of `rusoto_mock`.
8. Run `publish-services.sh` in the `rusoto/services` dir. *Warning*: takes >4 hours on a low end Macbook. The script can be run again if an issue comes up without problems - crates.io prevents republishing.
9. Tag master branch with the new version.  Example: `git tag -a rusoto-v0.41.0 -m "Rusoto 0.41.0 release."` then `git push --tags origin`.

### Git tags

Due to multiple crates being in the repo, releases for each crate will be in the format `crate-vmajor.minor.patch`. Rusoto core, service crates, credentials and `rusoto_mock` will all have the same versions for a new release:

Examples:

* `rusoto-v0.42.0`
* `credentials-v0.42.0`
* `signature-v0.42.0`
* `mock-v0.42.0`

When bug fixes for a crate are published, all crates get a new release.

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

#### Mdbook docs

See [the Rusoto mdbook project](https://github.com/rusoto/rusoto.github.io).
