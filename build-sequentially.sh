#!/bin/sh

if [[ "$TRAVIS_RUST_VERSION" == nightly* ]]; then
    export CARGO_ARGS="--no-default-features -vv"
fi

echo "running cargo build --$CARGO_ARGS"
cargo build --$CARGO_ARGS

for service in $(grep "all =" Cargo.toml | sed -e "s/all = \[//g" | sed -e "s/\]//g" | tr ", " "\n" | sed -e "s/\"//g")
do
echo "running cargo build --features '$service' --$CARGO_ARGS"
cargo build --features $service --$CARGO_ARGS
echo "running cargo test --features '$service' --lib --$CARGO_ARGS"
cargo test --features $service --lib --$CARGO_ARGS
done