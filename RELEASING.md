## Release trains

For pre-1.0.0:

* Targeting one release a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Publishing walkthrough:

1. Make a pull request that bumps version numbers for `rusoto_core`, `rusoto_credential` and `rusoto_mock` if needed and each service that changed since previous release.  Service versions are in the `services.json` file in the codegen project. Otherwise they are in the `Cargo.toml` files for each project.  Make sure the root Rusoto README example gets updated with the new version.
2. Merge release PR.
3. Publish new version of `rusoto_credential` if changes have been made to it.
4. Publish new version of `rusoto_mock` if changes have been made to it.
5. Publish new version of `rusoto_core` if it changes have been made to it.
6. Run `publish-services.sh` in the `rusoto/services` dir. *Warning*: takes >2 hours on a low end Macbook. The script can be run again if an issue comes up without problems - crates.io prevents republishing.
7. Tag master branch with the new version.  Example: `git tag -a rusoto-v0.21.0 -m "Rusoto 0.21.0 release."` then `git push --tags origin`.

### Git tags

Due to multiple crates being in the repo, releases for each crate will be in the format `crate-vmajor.minor.patch`.

Examples:

* `rusoto-v0.21.0`
* `credentials-v0.3.0`
* `mock-v0.27.0`

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

#### API docs

API docs are on Github Pages at [https://rusoto.github.io/rusoto](https://rusoto.github.io/rusoto).

TravisCI builds and publishes the `gh-pages` branch automatically when changes are merged into master.

#### Gitbook docs

See [the Rusoto gitbook project](https://github.com/rusoto/rusoto.github.io).
