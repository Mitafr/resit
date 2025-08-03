.PHONY: all build run test clean fmt lint example coverage

CARGO ?= cargo
RUST_LOG = "info"

all: build

build:
	$(CARGO) build --color always

build-release:
	$(CARGO) build --release --color always

test:
	$(CARGO) nextest run --color always

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

clean:
	$(CARGO) clean

coverage:
	$(CARGO) llvm-cov nextest --lcov --output-path ./target/lcov.info 

coverage-report: 
	$(CARGO) llvm-cov nextest --color always

example-%:
	RUST_LOG=${RUST_LOG} $(CARGO) run --example $*
