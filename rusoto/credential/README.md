# Rusoto credential

This crate contains the functionality necessary for loading and managing AWS
credentials for API requests. It was designed for use in
[Rusoto](https://github.com/rusoto/rusoto).

## Development

The Rusoto credential crate is packaged and published separately from the Rusoto
crate itself. The crate does not depend on Rusoto itself. The credential crate
is published as a versioned dependency of Rusoto on crates.io.