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

This splitting of the Rusoto crate is still in progress and the [Cargo.toml](Cargo.toml)
file requires Rusoto to be checked out in a sibling directory.  The plan is to
split things cleanly so Rusoto helpers will use a published Rusoto crate as a
dependency.
