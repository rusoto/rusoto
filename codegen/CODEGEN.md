# Code generation

Rust types for the core AWS APIs are generated from service definitions in the
[botocore](https://github.com/boto/botocore) library. The [botocore_parser.py](botocore_parser.py)
program parses a JSON API definition file and outputs Rust code:

```bash
./botocore_parser.py path/to/some.json ClientTypeName > some_module.rs
```

## SQS walkthrough

This is a guide to how SQS was added to Rusoto.

1.  If you haven't already, initialize and update the Git submodule for botocore:

    ```bash
    git submodule init
    git submodule update
    ```

2.  Generate Rust code for the API:

    ```bash
    ./codegen/botocore_parser.py codegen/botocore/botocore/data/sqs/2012-11-05/service-2.json SQSClient > codegen/sqs.rs
    ```

3.  Create `src/sqs.rs` with the following contents:

    ```rust
    use credentials::*;
    use xml::*;
    use signature::*;
    use params::*;
    use error::*;
    use xmlutil::*;
    use std::str::FromStr;
    use regions::*;

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sqs.rs"));
    ```

    This will include the generated Rust code in the sqs module of the crate, and import items
    from other modules in the crate that are needed by the generated code.

4.  Add any desired code to the module, such as a higher level wrapper type:

    ```rust
    /// High level SQS client that wraps the generated SQSClient.
    pub struct SQSHelper<'a> {
      client: SQSClient<'a>
    }

    impl<'a> SQSHelper<'a> {
      /// Creates a new SQS helper
      pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> SQSHelper<'a> {
        SQSHelper { client: SQSClient::new(credentials, region) }
      }

      /// Lists queues
      pub fn list_queues(&mut self) -> Result<ListQueuesResult, AWSError> {
        self.client.list_queues(&ListQueuesRequest::default())
      }

      // ...
    }
    ```

5.  Declare the module publicly in [lib.rs](../src/lib.rs):

    ```rust
    pub mod sqs;
    ```

6.  Test compilation with `cargo build`.

## TODO

* There are multiple different types of API styles used by AWS.  Right now the parser
only handles one of them properly, which is seen in the SQS code.  This should be
fixed so code can be generated from all of the API definitions from botocore.

* The helper functions aren't auto generated.

* integration tests in tests/servicename.rs and add service type in cargo.toml to
allow running just that service's integration tests.
