# Supported Platforms

GoVM aims to support all platforms that Rust supports. This document lists the target platforms and their support status.

## Tier 1 Platforms (Fully Supported)

These platforms are tested and guaranteed to work:

| Platform | Target | Status | Artifact |
|----------|--------|--------|----------|
| Linux x86_64 (GNU) | `x86_64-unknown-linux-gnu` | ✅ | `.tar.gz` |
| Linux x86_64 (musl) | `x86_64-unknown-linux-musl` | ✅ | `.tar.gz` |
| Linux ARM64 (GNU) | `aarch64-unknown-linux-gnu` | ✅ | `.tar.gz` |
| Linux ARM64 (musl) | `aarch64-unknown-linux-musl` | ✅ | `.tar.gz` |
| Linux ARMv7 | `armv7-unknown-linux-gnueabihf` | ✅ | `.tar.gz` |
| macOS x86_64 | `x86_64-apple-darwin` | ✅ | `.tar.gz` |
| macOS ARM64 | `aarch64-apple-darwin` | ✅ | `.tar.gz` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | ✅ | `.zip` |
| Windows x86 | `i686-pc-windows-msvc` | ✅ | `.zip` |
| Windows ARM64 | `aarch64-pc-windows-msvc` | ✅ | `.zip` |

## Tier 2 Platforms (Best Effort)

These platforms are built but may have limited testing:

### Additional Linux Architectures

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| Linux x86 | `i686-unknown-linux-gnu` | 🟡 | 32-bit x86 |
| Linux x86 (musl) | `i686-unknown-linux-musl` | 🟡 | 32-bit x86 with musl |
| Linux ARM | `arm-unknown-linux-gnueabihf` | 🟡 | ARMv6/ARMv7 |
| Linux ARM (musl) | `arm-unknown-linux-musleabihf` | 🟡 | ARM with musl |
| Linux Thumb | `thumbv7neon-unknown-linux-gnueabihf` | 🟡 | ARM with NEON |

### MIPS Architectures

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| Linux MIPS | `mips-unknown-linux-gnu` | 🟡 | Big-endian MIPS |
| Linux MIPS (LE) | `mipsel-unknown-linux-gnu` | 🟡 | Little-endian MIPS |
| Linux MIPS64 | `mips64-unknown-linux-gnuabi64` | 🟡 | 64-bit MIPS |
| Linux MIPS64 (LE) | `mips64el-unknown-linux-gnuabi64` | 🟡 | 64-bit MIPS LE |

### PowerPC Architectures

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| Linux PowerPC | `powerpc-unknown-linux-gnu` | 🟡 | 32-bit PowerPC |
| Linux PowerPC64 | `powerpc64-unknown-linux-gnu` | 🟡 | 64-bit PowerPC BE |
| Linux PowerPC64LE | `powerpc64le-unknown-linux-gnu` | 🟡 | 64-bit PowerPC LE |

### Other Architectures

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| Linux RISC-V | `riscv64gc-unknown-linux-gnu` | 🟡 | RISC-V 64-bit |
| Linux IBM Z | `s390x-unknown-linux-gnu` | 🟡 | IBM System Z |
| Linux SPARC64 | `sparc64-unknown-linux-gnu` | 🟡 | SPARC 64-bit |
| Linux LoongArch | `loongarch64-unknown-linux-gnu` | 🟡 | Loongson 64-bit |

### BSD Systems

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| FreeBSD | `x86_64-unknown-freebsd` | 🟡 | FreeBSD x86_64 |
| NetBSD | `x86_64-unknown-netbsd` | 🟡 | NetBSD x86_64 |

### Android

| Platform | Target | Status | Notes |
|----------|--------|--------|-------|
| Android ARM64 | `aarch64-linux-android` | 🟡 | Android ARM64 |
| Android ARMv7 | `armv7-linux-androideabi` | 🟡 | Android ARMv7 |
| Android x86_64 | `x86_64-linux-android` | 🟡 | Android x86_64 |
| Android x86 | `i686-linux-android` | 🟡 | Android x86 |

## Legend

- ✅ **Fully Supported**: Tested and verified to work
- 🟡 **Best Effort**: Compiled but limited testing
- ❌ **Not Supported**: Known not to work

## Platform-Specific Notes

### Linux (GNU vs musl)

- **GNU libc**: Standard for most Linux distributions (Ubuntu, Debian, Fedora, etc.)
- **musl libc**: Lightweight libc for Alpine Linux and embedded systems

If you're unsure which to use:
- Use `gnu` for standard desktop/server Linux
- Use `musl` for Alpine Linux or minimal containers

### Windows MSVC vs GNU

We currently only provide MSVC builds for Windows as they are the most compatible.

### macOS Universal Binary

While we provide separate x86_64 and ARM64 builds for macOS, macOS can run x86_64 binaries on Apple Silicon via Rosetta 2.

## Requesting Additional Platforms

If you need a platform that's not listed here, please:

1. Check if Rust supports it: `rustup target list`
2. Open an issue with the target name and use case
3. We may add it to the build matrix

## Building for Unsupported Platforms

You can always build GoVM from source for any Rust-supported platform:

```bash
# Install the target
rustup target add <target-triple>

# Build
cargo build --release --target <target-triple>
```

For cross-compilation, we recommend using [cross](https://github.com/cross-rs/cross):

```bash
cargo install cross
cross build --release --target <target-triple>
```
