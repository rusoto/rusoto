#!/bin/sh

cargo build; cargo run &

cd ../credential ; sleep 5 ; cargo test -- --ignored ; killall credential_service_mock
