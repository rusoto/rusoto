# Code generation

Rust types for the core AWS APIs are generated from service definitions in the
[botocore](https://github.com/boto/botocore) library. The [botocore_parser.py](botocore_parser.py)
program parses a JSON API definition file and outputs Rust code:

```bash
./botocore_parser.py path/to/some.json ClientTypeName > some_module.rs
```

This executable is not intended to be run directly by a developer.
It is invoked automatically during the build process (`cargo build`).

## SQS walkthrough

This is a guide to how SQS was added to Rusoto.

1.  If you haven't already, initialize and update the Git submodule for botocore:

    ```bash
    git submodule init
    git submodule update
    ```

2.  Add a code generation call for it inside `build.rs`:

    ```rust
    fn main() {
      // ...

      // SQS
      generate(
          "codegen/botocore/botocore/data/sqs/2012-11-05/service-2.json",
          "SQSClient",
          out_path,
          "sqs",
      )

      // ...
    }
    ```

3.  Create `src/sqs.rs` with the following contents:

    ```rust
    include!(concat!(env!("OUT_DIR"), "/sqs.rs"));
    ```

    This will include the generated Rust code in the sqs module of the crate, and import items
    from other modules in the crate that are needed by the generated code.

4.  Run `cargo build`. You will get compiler errors about unresolvable names.
    These are the types from the other Rusoto modules that the new module depends on.
    One by one, import them into `src/sqs.rs`, running `cargo build` again after each import to
    see what the next one is. Continue until all the required types have been imported and
    the build succeeds. You'll end up with something that looks like this:

    ```rust
    #![allow(unused_variables, unused_mut)]

    use std::str::FromStr;

    use xml::EventReader;

    use credentials::AWSCredentialsProvider;
    use error::AWSError;
    use params::{Params, SQSParams};
    use regions::Region;
    use signature::SignedRequest;
    use xmlutil::{
        Next,
        Peek,
        XmlParseError,
        XmlResponseFromAws,
        characters,
        end_element,
        peek_at_name,
        start_element,
    };
    ```

    For the time being, we are suppressing warnings about unused variables and unnecessarily
    mutable variables, but this will be improved in future versions of generated code.

4.  Add any desired helper code to the module, such as a higher level wrapper type:

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

7.  Add comments to `lib.rs` and `README.rd`, and update `Cargo.toml` to indicate that Rusoto now supports this new AWS service.

## TODO

* There are multiple different types of API styles used by AWS.
  Right now the doesn't handle all of them.
  Eventually all services with data in the botocore code should be supported by our code generation.
* The helper functions aren't auto generated.
