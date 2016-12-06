## Release trains

For pre-1.0.0:

* Targeting two releases a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Rusoto dependencies

Rusoto depends on crates such as `rusoto_credential` and `rusoto_codegen`.  These are usually published before publishing
the main Rusoto crate.

### Publishing walkthrough:

1. Bump version numbers and versions required in cargo.toml files for: `rusoto_credential` (if needed), `rusoto_codegen` (if needed) and main `rusoto` crate.
2. For credential and codegen crates, run `cargo publish --dry-run --allow-dirty` to simulate a publish. This will catch errors such as malformed Cargo.toml or missing required fields.
3. If no errors, commit to master.
4. Publish the credential and/or codegen crates with `cargo publish` in their directories.
5. Last check for main crate, run from the root of the Rusoto project: `cargo publish --dry-run`
6. If good, publish new version of Rusoto: `cargo publish`.
7. Tag master, push tags to github.

### Git tags

To release version 0.4.0:

1. Use an annotated tag on the commit with the version bump: `git tag -a v0.4.0 -m "0.4.0 release."`
2. Push changes, including tags, to Github: `git push --tags origin`.

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

### Documentation

Docs are on Github Pages at [https://rusoto.github.io/rusoto](https://rusoto.github.io/rusoto).

TravisCI builds and publishes the `gh-pages` branch automatically when changes are merged into master.
