## Release procedure for Rusoto

### Semantic versioning

Rusoto uses [semantic versioning 2.0.0](http://semver.org/).  

### Git tags

To release version 0.4.0 of Rusoto:

1. On master, bump the version in Cargo.toml to the new version.  In this example, we'll set it to 0.4.0.
2. Commit to master.
3. Use an annotated tag on the commit with the version bump: `git tag -a v0.4.0 -m "0.4.0 release."`
4. Push changes, including tags, to Github: `git push --tags origin`.

### Crate publishing

To publish on crates.io:

1. `cargo package`
2. `cargo publish`
