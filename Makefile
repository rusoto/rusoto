# System Setup
SHELL = bash


.PHONY: help
help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

service_crategen/botocore/.git:
	git submodule update --init --recursive

.PHONY: botocore
botocore: service_crategen/botocore/.git

##@ Development

.PHONY: clean
clean: ## run cargo clean
	cargo +$$RUST_VERSION clean

.PHONY: fmt
fmt: ## run cargo format
	cargo +$$RUST_VERSION fmt

.PHONY: generate
generate: botocore ## regenerate all services as described in ./services.json
	(cd service_crategen && cargo +$$RUST_VERSION run -- generate -c ./services.json -o ../rusoto/services)

.PHONY: build
build: generate ## build all services after generating them
	cargo +$$RUST_VERSION build --features all

.PHONY: docs
docs: ## run cargo doc
	cargo +$$RUST_VERSION doc --all --no-deps

##@ Testing

.PHONY: unit_test
unit_test: ## run cargo test (with doctests, can be slow!)
	cargo +$$RUST_VERSION test --all

# Doctests can be very slow to compile and run. This option lets us skip those if needed.
.PHONY: unit_test_no_doctests
unit_test_no_doctests: ## run cargo test (without doctests)
	cargo +$$RUST_VERSION test --all --lib

.PHONY: skeptical
skeptical: ## run skeptic on mdbook markdown files to verify Rust source code
	(cd skeptical && cargo +$$RUST_VERSION test)

.PHONY: integration_test
integration_test: ## run integration tests against AWS using your account (charges may incur)
	(cd integration_tests && cargo +$$RUST_VERSION test --features all -- --test-threads 1)

.PHONY: check_integration_test
check_integration_test: ## run cargo check on integration tests
	(cd integration_tests && cargo +$$RUST_VERSION check --tests --features all)

.PHONY: rustls_unit_test_no_doctests
rustls_unit_test_no_doctests: ## run unit tests for each service with rustls enabled, without doctests
	(cd rusoto/core && cargo +$$RUST_VERSION test --no-default-features --features=rustls --lib)
	(cd rusoto/services && ./rustls-unit-test-no-doctests.sh $$RUST_VERSION)

.PHONY: rustls_unit_test
rustls_unit_test: ## run unit tests for each service with rustls enabled
	(cd rusoto/core && cargo +$$RUST_VERSION test --no-default-features --features=rustls)
	(cd rusoto/services && ./rustls-unit-test.sh $$RUST_VERSION)

.PHONY: check_service_defintions
check_service_defintions: botocore ## check for missing and outdated services
	(cd service_crategen && cargo +$$RUST_VERSION run -- check -c ./services.json)

##@ Performance

.PHONY: time_credentials
time_credentials: ## Measure the time to build the credentials crate
	(cd rusoto/credential && cargo clean --package rusoto_credential && touch src/lib.rs && time cargo +$$RUST_VERSION build)

.PHONY: bench_s3
bench_s3: ## run S3 benchmarks
	(cd rusoto/services/s3 && cargo +nightly bench)

.PHONY: credential_integration_test
credential_integration_test: ## Run credentials integration tests
	(cd rusoto/credential_service_mock && ./run-and-test.sh )
