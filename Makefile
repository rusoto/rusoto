# System Setup
SHELL = bash

RUST_VERSION ?= stable

.PHONY: clean
clean:
	cargo +$$RUST_VERSION clean

.PHONY: fmt
fmt:
	cargo +$$RUST_VERSION fmt

.PHONY: generate
generate:
	(cd service_crategen && cargo +$$RUST_VERSION run -- generate -c ./services.json -o ../rusoto/services)

.PHONY: build
build: generate
	cargo +$$RUST_VERSION build --features all

.PHONY: unit_test
unit_test:
	cargo +$$RUST_VERSION test2 --all