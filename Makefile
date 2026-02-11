# GoVM Makefile
# Usage: make [target]

.PHONY: all build release clean test install uninstall

# Default target
all: build

# Build for current platform
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist/

# Install locally (Unix)
install: build
	@echo "Installing govm to /usr/local/bin..."
	@sudo cp target/release/govm /usr/local/bin/
	@echo "✅ govm installed successfully"
	@echo "Run 'govm --help' to get started"

# Uninstall (Unix)
uninstall:
	@echo "Uninstalling govm..."
	@sudo rm -f /usr/local/bin/govm
	@echo "✅ govm uninstalled"

# Build for all platforms (requires cross)
release-all: release-linux release-macos release-windows
	@echo "✅ All builds complete"
	@mkdir -p dist
	@cp target/*/release/govm-* dist/ 2>/dev/null || true
	@ls -la dist/

# Linux builds
release-linux:
	@echo "Building for Linux..."
	cargo build --release --target x86_64-unknown-linux-gnu
	mkdir -p dist
	tar czvf dist/govm-x86_64-unknown-linux-gnu.tar.gz -C target/x86_64-unknown-linux-gnu/release govm

release-linux-musl:
	@echo "Building for Linux (musl)..."
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p dist
	tar czvf dist/govm-x86_64-unknown-linux-musl.tar.gz -C target/x86_64-unknown-linux-musl/release govm

release-linux-arm64:
	@echo "Building for Linux ARM64..."
	cargo build --release --target aarch64-unknown-linux-gnu
	mkdir -p dist
	tar czvf dist/govm-aarch64-unknown-linux-gnu.tar.gz -C target/aarch64-unknown-linux-gnu/release govm

# macOS builds
release-macos:
	@echo "Building for macOS..."
	cargo build --release --target x86_64-apple-darwin
	mkdir -p dist
	tar czvf dist/govm-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release govm

release-macos-arm64:
	@echo "Building for macOS ARM64..."
	cargo build --release --target aarch64-apple-darwin
	mkdir -p dist
	tar czvf dist/govm-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release govm

# Windows builds (cross compilation from Linux/macOS requires mingw)
release-windows:
	@echo "Building for Windows..."
	cargo build --release --target x86_64-pc-windows-msvc 2>/dev/null || \
	cargo build --release --target x86_64-pc-windows-gnu
	mkdir -p dist
	cd target/x86_64-pc-windows-*/release && zip ../../../dist/govm-x86_64-pc-windows.zip govm.exe

# Setup cross-compilation targets
setup:
	@echo "Setting up Rust targets..."
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-musl
	rustup target add armv7-unknown-linux-gnueabihf
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-pc-windows-gnu
	@echo "✅ Targets installed"

# Check code formatting and lint
lint:
	cargo fmt --check
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Show help
help:
	@echo "GoVM Makefile targets:"
	@echo ""
	@echo "  make build          - Build for current platform"
	@echo "  make test           - Run tests"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make install        - Install govm to /usr/local/bin (Unix)"
	@echo "  make uninstall      - Uninstall govm (Unix)"
	@echo "  make release-all    - Build for all platforms"
	@echo "  make setup          - Install Rust targets for cross-compilation"
	@echo "  make lint           - Check formatting and run clippy"
	@echo "  make fmt            - Format code"
	@echo ""
