## Release trains

For pre-1.0.0:

* Targeting two releases a month for minor versions.
* Regression bug fixes will be released ASAP on best effort for maintainers.  For example, a regression in 0.9.0 means 0.9.1 is released ASAP instead of waiting for the next release train.

## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).

### Codegen crate

The `rusoto_codegen` crate is published separately from the main `rusoto` crate and must be published first since the main crate depends on it.  

The release procedures for the codegen crate is the same as described below except:

* Tags are only applied for the root `rusoto` crate.
* Before the `rusoto` crate is published, its dependency on `rusoto_codegen` should be updated to reflect the newly published version.

### Git tags

To release version 0.4.0:

1. Ensure `cargo package` completes as expected.  It's a good idea to inspect the related output.
2. On master, bump the version in Cargo.toml to the new version.  In this example, we'll set it to 0.4.0.
3. Update Cargo.toml to reflect the new version of the `rusoto_codegen` crate that was just published.
4. Commit to master.
5. Use an annotated tag on the commit with the version bump: `git tag -a v0.4.0 -m "0.4.0 release."`
6. Push changes, including tags, to Github: `git push --tags origin`.

### Crate publishing

To publish on crates.io:

1. `cargo package`
2. `cargo publish`

### Release notes

Add a list of user-facing changes to a new release for the tagged version on GitHub: https://github.com/rusoto/rusoto/releases

### Documentation

Docs are on Github Pages at [https://rusoto.github.io/rusoto](https://rusoto.github.io/rusoto).

TravisCI builds and publishes the `gh-pages` branch automatically when changes are merged into master.
