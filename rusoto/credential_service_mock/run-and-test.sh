#!/bin/sh

cargo build; cargo run &

cd ../credential ; cargo test ; killall credential_service_mock
