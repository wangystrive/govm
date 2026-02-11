# GoVM Makefile
# Build for Go-supported platforms only

.PHONY: all build test clean install

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

# Uninstall (Unix)
uninstall:
	@echo "Uninstalling govm..."
	@sudo rm -f /usr/local/bin/govm
	@echo "✅ govm uninstalled"

# Release builds for all Go-supported platforms
release: release-linux release-darwin release-windows release-freebsd
	@echo "✅ All releases built"
	@mkdir -p dist
	@cp target/*/release/govm-*.tar.gz dist/ 2>/dev/null || true
	@cp target/*/release/govm-*.zip dist/ 2>/dev/null || true
	@ls -la dist/

# Linux releases (matching Go platforms)
release-linux:
	@echo "Building for Linux..."
	cargo install cross --locked 2>/dev/null || true
	cross build --release --target x86_64-unknown-linux-gnu
	cross build --release --target i686-unknown-linux-gnu
	cross build --release --target aarch64-unknown-linux-gnu
	cross build --release --target arm-unknown-linux-gnueabihf
	mkdir -p dist
	cd target/x86_64-unknown-linux-gnu/release && tar czvf ../../../dist/govm-linux-amd64.tar.gz govm
	cd target/i686-unknown-linux-gnu/release && tar czvf ../../../dist/govm-linux-386.tar.gz govm
	cd target/aarch64-unknown-linux-gnu/release && tar czvf ../../../dist/govm-linux-arm64.tar.gz govm
	cd target/arm-unknown-linux-gnueabihf/release && tar czvf ../../../dist/govm-linux-armv6l.tar.gz govm

# macOS releases (matching Go platforms)
release-darwin:
	@echo "Building for macOS..."
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	mkdir -p dist
	cd target/x86_64-apple-darwin/release && tar czvf ../../../dist/govm-darwin-amd64.tar.gz govm
	cd target/aarch64-apple-darwin/release && tar czvf ../../../dist/govm-darwin-arm64.tar.gz govm

# Windows releases (matching Go platforms)
release-windows:
	@echo "Building for Windows..."
	cargo build --release --target x86_64-pc-windows-msvc
	cargo build --release --target i686-pc-windows-msvc
	cargo build --release --target aarch64-pc-windows-msvc
	mkdir -p dist
	cd target/x86_64-pc-windows-msvc/release && zip ../../../dist/govm-windows-amd64.zip govm.exe
	cd target/i686-pc-windows-msvc/release && zip ../../../dist/govm-windows-386.zip govm.exe
	cd target/aarch64-pc-windows-msvc/release && zip ../../../dist/govm-windows-arm64.zip govm.exe

# FreeBSD release (matching Go platforms)
release-freebsd:
	@echo "Building for FreeBSD..."
	cargo install cross --locked 2>/dev/null || true
	cross build --release --target x86_64-unknown-freebsd
	mkdir -p dist
	cd target/x86_64-unknown-freebsd/release && tar czvf ../../../dist/govm-freebsd-amd64.tar.gz govm

# Setup Rust targets for Go-supported platforms
setup:
	@echo "Installing Rust targets for Go-supported platforms..."
	rustup target add x86_64-unknown-linux-gnu
	rustup target add i686-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-gnu
	rustup target add arm-unknown-linux-gnueabihf
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-pc-windows-msvc
	rustup target add i686-pc-windows-msvc
	rustup target add aarch64-pc-windows-msvc
	rustup target add x86_64-unknown-freebsd
	@echo "✅ All targets installed"

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
	@echo "  make release        - Build for all Go-supported platforms"
	@echo "  make setup          - Install Rust targets"
	@echo "  make lint           - Check formatting and run clippy"
	@echo "  make fmt            - Format code"
	@echo ""
	@echo "Supported platforms (matching Go official releases):"
	@echo "  Linux:   amd64, 386, arm64, armv6l"
	@echo "  macOS:   amd64, arm64"
	@echo "  Windows: amd64, 386, arm64"
	@echo "  FreeBSD: amd64"
