## Release trains

For pre-1.0.0:

* Targeting two releases a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Codegen crate

The `rusoto_codegen`crate is published separately from the main `rusoto` crate, and must be published first since the main crate depends on it.  

The release procedures for the crates are identical as described below, with the caveat that when the `rusoto` crate is published, its dependency on `rusoto_codegen` should be updated to reflect the newly published version.

### Git tags

To release version 0.4.0:

1. On master, bump the version in Cargo.toml to the new version.  In this example, we'll set it to 0.4.0.
2. If publishing the `rusoto` crate, also update Cargo.toml to reflect the new version of the `rusoto_codegen` crate that was just published.
3. Commit to master.
4. Use an annotated tag on the commit with the version bump: `git tag -a v0.4.0 -m "0.4.0 release."`
5. Push changes, including tags, to Github: `git push --tags origin`.

### Crate publishing

To publish on crates.io:

1. `cargo package`
2. `cargo publish`

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

### Documentation

Docs are on Github Pages at [https://rusoto.github.io/rusoto](https://rusoto.github.io/rusoto).

A helper script is included to make this less painful until our CI system handles it.  Run `docgen.sh` to
use [github pages import](https://github.com/davisp/ghp-import) to upload them to the gh-pages branch.  Install
ghp-import if you don't have it:

`pip install ghp-import`
