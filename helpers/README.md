# Rusoto helpers

This crate contains the high level helper functions for the AWS services
implemented in [Rusoto](https://github.com/rusoto/rusoto).

For example, instead of using the lower level functions from Rusoto, like this:


```rust
let client = SQSClient::new(credentials, region);
try!(client.list_queues(&ListQueuesRequest::default()));
```

the higher level functions can be used:

```rust
let mut sqs = SQSHelper::new(provider, &region);
sqs.list_queues();
```

The `SQSHelper` object takes care of keeping the provider and region, so it can be reused easier.

## Development

The Rusoto helpers crate is packaged and published separately from the Rusoto crate itself.  The  [Cargo.toml](Cargo.toml)
file has a dependency to Rusoto in the parent directory `..`, as these crates share a common git repository.  This local dependency helps to synchronize development of features in the helper crate with any changes necessary in the parent.  The helpers crate is published with a versioned dependency on the Rusoto crate as provided by crates.io.
