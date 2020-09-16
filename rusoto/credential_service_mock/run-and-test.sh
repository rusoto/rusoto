#!/bin/sh

cargo build; cargo run &

cd ../credential ; sleep 5 ; cargo test -- --ignored ; STATUS=$? ; killall credential_service_mock
exit $STATUS
