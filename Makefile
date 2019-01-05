# System Setup
SHELL = bash

.PHONY: clean
clean:
	-@cargo +stable clean

.PHONY: fmt
fmt:
	-@cargo +stable fmt

.PHONY: generate
generate:
	-@(cd service_crategen && cargo +stable run -- generate -c ./services.json -o ../rusoto/services)

.PHONY: build
build: generate
	-@cargo +stable build --features all

.PHONY: unit_test
unit_test:
	-@cargo +stable test --all