#!/bin/bash
set -Eeu

export RUST_BACKTRACE=1
export PATH=${HOME}/.cargo/bin:${PATH}
export GIT_ROOT=$(git rev-parse --show-toplevel)

run-parts --verbose --exit-on-error --regex '^test_' .semaphoreci 2>&1
