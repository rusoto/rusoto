# Code generation

## Overview

The [botocore_parser.py](botocore_parser.py) Python file parses a JSON API definition
file and outputs Rust code.  Example:

```bash
./botocore_parser path/to/some.json ClientClassName > some_module.rs
```

JSON definitions come from the [botocore](https://github.com/boto/botocore) repo, under the [botocore/data](https://github.com/boto/botocore/tree/develop/botocore/data) directory.  For each service, copy its definition file by hand to `codegen/` and then follow this document.

### SQS walkthrough

This is a guide to how SQS was added to Rusoto.  After placing the sqs.json file from the above
botocore location into `codegen/sqs.json`:

1. In the `codegen` directory:

```bash
$ ./botocore_parser.py sqs.json SQSClient > sqs.rs
```

2. Create `src/sqs.rs` file.  This will be a wrapper around the generated code and
where helper functions can go.  This line will bring in the specified file as source code:

```rust
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sqs.rs"));
```

3.  Fill out remainder of `srs/sqs.rs` with constructors for new class and helper functions:

```rust
/// Easier to use SQS client: wraps SQSClient class.
pub struct SQSHelper<'a> {
	client: SQSClient<'a>
}

impl<'a> SQSHelper<'a> {
	/// Creates a new SQS helper
	pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> SQSHelper<'a> {
		SQSHelper { client: SQSClient::new(credentials, region) }
	}
```

Helper function example:

```rust
  /// Lists queues
	pub fn list_queues(&mut self) -> Result<ListQueuesResult, AWSError> {
		self.client.list_queues(&ListQueuesRequest::default())
	}
```

4.  Add the new module to publicly accessible modules for the library by adding the new class
to [lib.rs](../src/lib.rs):

```rust
pub mod sqs;
```

5.  Test compile with `cargo build`.

## TODO

* There are multiple different types of API styles used by AWS.  Right now the parser
only handles one of them properly, which is seen in the SQS code.  This should be
fixed so code can be generated from all of the API definitions from botocore.

* The helper functions aren't auto generated.
