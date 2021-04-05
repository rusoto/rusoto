# Additional CI tests

This directory contains test scripts that run integration tests against
Ceph and MinIO.

These tests were historically run on [Semaphore CI](https://semaphoreci.com),
but now run on GitHub Actions with the rest of the CI jobs.

## Setting Environment Variables

To ensure an environment variable is available in all steps, export it in run.sh.

## Adding More Tests/Scripts

1. Add a file that starts with `test_NN_` where `NN` is a number that decides the execution order.
2. Ensure file is executable (`chmod +x $FILE`)

:warning: Be careful when adding a script after `test_99_codegen.sh`, that step may change
`generated.rs` files.

## Executing Tests Locally

See [README for integration tests](/integration_tests/README.md).
