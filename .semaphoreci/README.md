# Semaphore CI

This directory contains test scripts run on [Semaphore CI](https://semaphoreci.com/matthewkmayer/rusoto).

## Setup On Semaphore CI

1. Create a Semaphore project
2. Select a *platform* in the setting with *native Docker* support
3. Have this command executed: `.semaphoreci/run.sh`

## Setting Environment Variables

To ensure an environment variable is available in all steps, export it in run.sh.

## Adding More Tests/Scripts

1. Add a file that starts with `test_NN_` where `NN` is a number that decides the execution order.
2. Ensure file is executable (`chmod +x $FILE`)

:warning: Be careful when adding a script after `test_99_codegen.sh`, that step may change
`generated.rs` files.

## Executing Tests Locally

See [README for integration tests](/integration_tests/README.md).
