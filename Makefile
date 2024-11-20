# Variables
BINARY_NAME = chess_pgn_parser
PACKAGE_NAME = chess_parser

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	cargo build --release

# Run the program
.PHONY: run
run:
	cargo run --release -- $(ARGS)

# Run tests
.PHONY: test
test:
	cargo test

# Format the code
.PHONY: format
format:
	cargo fmt

# Lint the code with Clippy
.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

# Check formatting and linting
.PHONY: check
check: format clippy

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Generate documentation
.PHONY: docs
docs:
	cargo doc --open

# Prepare for commit (format, clippy, test)
.PHONY: pre-commit
pre-commit: format clippy test

# Install the binary
.PHONY: install
install:
	cargo install --path .

# Uninstall the binary
.PHONY: uninstall
uninstall:
	cargo uninstall $(PACKAGE_NAME)

# Update dependencies
.PHONY: update
update:
	cargo update

# Help
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make build        - Build the project in release mode"
	@echo "  make run ARGS='...' - Run the program with arguments"
	@echo "  make test         - Run tests"
	@echo "  make format       - Format the code with cargo fmt"
	@echo "  make clippy       - Lint the code with Clippy"
	@echo "  make check        - Run format and clippy"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docs         - Generate and open documentation"
	@echo "  make pre-commit   - Run checks before committing"
	@echo "  make install      - Install the binary locally"
	@echo "  make uninstall    - Uninstall the binary"
	@echo "  make update       - Update dependencies"
	@echo "  make help         - Show this help message"

# Default make target when no arguments are given
.DEFAULT_GOAL := help
