# Supported Platforms

GoVM supports the same platforms as the official Go distribution.

## Official Go Platforms

| OS | Architecture | Rust Target | Download Filename |
|----|-------------|-------------|-------------------|
| **Linux** | amd64 (x86_64) | `x86_64-unknown-linux-gnu` | `govm-linux-amd64.tar.gz` |
| **Linux** | 386 (x86) | `i686-unknown-linux-gnu` | `govm-linux-386.tar.gz` |
| **Linux** | arm64 | `aarch64-unknown-linux-gnu` | `govm-linux-arm64.tar.gz` |
| **Linux** | armv6l | `arm-unknown-linux-gnueabihf` | `govm-linux-armv6l.tar.gz` |
| **macOS** | amd64 (Intel) | `x86_64-apple-darwin` | `govm-darwin-amd64.tar.gz` |
| **macOS** | arm64 (Apple Silicon) | `aarch64-apple-darwin` | `govm-darwin-arm64.tar.gz` |
| **Windows** | amd64 | `x86_64-pc-windows-msvc` | `govm-windows-amd64.zip` |
| **Windows** | 386 | `i686-pc-windows-msvc` | `govm-windows-386.zip` |
| **Windows** | arm64 | `aarch64-pc-windows-msvc` | `govm-windows-arm64.zip` |
| **FreeBSD** | amd64 | `x86_64-unknown-freebsd` | `govm-freebsd-amd64.tar.gz` |

## Platform Notes

### Linux

- **amd64**: Most common desktop/server Linux (Intel/AMD 64-bit)
- **386**: 32-bit x86 systems (older hardware)
- **arm64**: ARM 64-bit (Raspberry Pi 4, AWS Graviton, Apple M1 Linux VMs)
- **armv6l**: ARM 32-bit (Raspberry Pi Zero, older ARM boards)

### macOS

- **amd64**: Intel-based Macs (2015-2020)
- **arm64**: Apple Silicon Macs (M1, M2, M3)

### Windows

- **amd64**: 64-bit Windows (most common)
- **386**: 32-bit Windows
- **arm64**: Windows on ARM (Surface Pro X, etc.)

### FreeBSD

- **amd64**: 64-bit FreeBSD systems

## Installation

### Linux/macOS/FreeBSD

```bash
# Download the appropriate file for your platform
wget https://github.com/wangystrive/govm/releases/latest/download/govm-linux-amd64.tar.gz

# Extract
tar xzf govm-linux-amd64.tar.gz

# Move to PATH
sudo mv govm /usr/local/bin/
```

### Windows

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/wangystrive/govm/releases/latest/download/govm-windows-amd64.zip" -OutFile "govm.zip"

# Extract
Expand-Archive -Path "govm.zip" -DestinationPath "$env:LOCALAPPDATA\govm\bin"

# Add to PATH (User)
[Environment]::SetEnvironmentVariable("Path", "$env:LOCALAPPDATA\govm\bin;$env:Path", "User")
```

## Verification

```bash
# Check version
govm --version

# List installed Go versions
govm list

# Install and use Go
govm install 1.21.5
govm use 1.21.5
go version
```

## Comparison with Go Releases

GoVM's release artifacts follow the same naming convention as Go official releases:

| Component | Go | GoVM |
|-----------|-----|------|
| Linux amd64 | `go1.21.5.linux-amd64.tar.gz` | `govm-linux-amd64.tar.gz` |
| macOS arm64 | `go1.21.5.darwin-arm64.tar.gz` | `govm-darwin-arm64.tar.gz` |
| Windows amd64 | `go1.21.5.windows-amd64.zip` | `govm-windows-amd64.zip` |

This makes it easy to script installations for both tools.

## Building from Source

If your platform is not listed above but Go supports it, you can build from source:

```bash
# Install Rust target
rustup target add <target-triple>

# Build
cargo build --release --target <target-triple>
```

For cross-compilation:

```bash
cargo install cross
cross build --release --target <target-triple>
```
