# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the project
run: build
	cargo run --release

# Clean the project
clean:
	cargo clean

# Platform-specific targets
.PHONY: linux mac windows

linux: build
	@echo "Building for Linux..."
	cargo build --release --target=x86_64-unknown-linux-musl

mac: build
	@echo "Building for macOS..."
	cargo build --release --target=x86_64-apple-darwin

windows: build
	@echo "Building for Windows..."
	cargo build --release --target=x86_64-pc-windows-gnu
