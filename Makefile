# Variables
TARGET = wasm32-wasip1
RELEASE_TARGET = ./target/$(TARGET)/release/bless_plugins.wasm
DEBUG_TARGET = ./target/$(TARGET)/debug/bless_plugins.wasm
FINAL_WASM = bless_plugins.wasm

# Default target
.PHONY: all
all: build

# Build release version
.PHONY: build
build: $(FINAL_WASM)

$(FINAL_WASM): $(RELEASE_TARGET)
	./javy init-plugin $< -o $@

$(RELEASE_TARGET):
	cargo build --target=$(TARGET) --release

# Build debug version
.PHONY: debug
debug: $(DEBUG_TARGET)
	javy init-plugin $< -o debug_$(FINAL_WASM)

$(DEBUG_TARGET):
	cargo build --target=$(TARGET)

# Check code without building
.PHONY: check
check:
	cargo check --target=$(TARGET)

# Run tests
.PHONY: test
test:
	cargo test

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Check formatting
.PHONY: fmt-check
fmt-check:
	cargo fmt -- --check

# Run clippy lints
.PHONY: lint
lint:
	cargo clippy --target=$(TARGET) -- -D warnings

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -f $(FINAL_WASM)
	rm -f debug_$(FINAL_WASM)

# Install required tools
.PHONY: install-tools
install-tools:
	rustup target add $(TARGET)
	@echo "Please install javy-cli manually from: https://github.com/bytecodealliance/javy"

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build        - Build release version (default)"
	@echo "  debug        - Build debug version"
	@echo "  check       - Check code without building"
	@echo "  test        - Run tests"
	@echo "  fmt         - Format code"
	@echo "  fmt-check   - Check code formatting"
	@echo "  lint        - Run clippy lints"
	@echo "  clean       - Clean build artifacts"
	@echo "  install-tools - Install required tools (except javy-cli)" 