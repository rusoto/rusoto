## Release trains

For pre-1.0.0:

* Targeting two releases a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Publishing walkthrough:

1. Make a pull request that bumps version numbers for `rusoto_core` if needed and each service that changed since previous release.  Make sure the README example gets updated with the new version.
2. After new release PR has been merged, publish `rusoto_core` if it had changes since last release.
3. Run `publish-services.sh` in the `rusoto/services` dir. *Warning*: takes 1.5 hours on a low end Macbook.
4. Tag master with the new version.  Example: `git tag -a rusoto-v0.21.0 -m "Rusoto 0.21.0 release."` then `git push --tags origin`.

### Git tags

Due to multiple crates being in the repo, releases for each crate will be in the format `crate-vmajor.minor.patch`.

Examples:

* `rusoto-v0.21.0`
* `credentials-v0.3.0`

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

#### API docs

API docs are on Github Pages at [https://rusoto.github.io/rusoto](https://rusoto.github.io/rusoto).

TravisCI builds and publishes the `gh-pages` branch automatically when changes are merged into master.

#### Gitbook docs

See [the Rusoto gitbook project](https://github.com/rusoto/rusoto.github.io).
