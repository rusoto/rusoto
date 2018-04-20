#!/bin/bash
set -Eeu

rustup toolchain install nightly
rustup component add rustfmt-preview --toolchain nightly
