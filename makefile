CARGO := cargo
PROJECT_NAME := my_rust_project
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
BIN_NAME := $(shell $(CARGO) metadata --no-deps --format-version 1 | jq -r '.packages[0].targets[] | select(.kind[] == "bin") | .name')

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

run:
	$(CARGO) run

run-release:
	$(CARGO) run --release

test:
	$(CARGO) test

bench:
	$(CARGO) bench

format:
	$(CARGO) fmt

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

check:
	$(CARGO) check

doc:
	$(CARGO) doc --no-deps --open

clean:
	$(CARGO) clean

package:
	$(CARGO) package

publish:
	$(CARGO) publish

install:
	$(CARGO) install --path .

update:
	$(CARGO) update

deps:
	$(CARGO) tree

size:
	du -sh $(RELEASE_DIR)/$(BIN_NAME)

strip:
	strip $(RELEASE_DIR)/$(BIN_NAME)

help:
	@echo "Available targets:"
	@echo "  build         - Build the project"
	@echo "  release       - Build the project in release mode"
	@echo "  run           - Run the project"
	@echo "  run-release   - Run the project in release mode"
	@echo "  test          - Run tests"
	@echo "  bench         - Run benchmarks"
	@echo "  format        - Format the code"
	@echo "  lint          - Lint the code"
	@echo "  check         - Type-check the code"
	@echo "  doc           - Generate documentation"
	@echo "  clean         - Clean build artifacts"
	@echo "  package       - Package for crates.io"
	@echo "  publish       - Publish to crates.io"
	@echo "  install       - Install locally"
	@echo "  update        - Update dependencies"
	@echo "  deps          - Show dependency tree"
	@echo "  size          - Show binary size"
	@echo "  strip         - Strip symbols from release binary"

.PHONY: all build release run run-release test bench format lint check doc clean package publish install update deps size strip help