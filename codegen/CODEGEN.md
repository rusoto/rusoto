# Code generation

## Overview

The [botocore_parser.py](botocore_parser.py) Python file parses a JSON API definition
file and outputs Rust code.  Example:

```bash
./botocore_parser path/to/some.json ClientClassName > some_module.rs
```

### SQS walkthrough

This is a guide to how SQS was added to Rusoto.  

## TODO

There are multiple different types of API styles used by AWS.  Right now the parser
only handles one of them properly, which is seen in the SQS code.  This should be
fixed so code can be generated from all of the API definitions from botocore.
