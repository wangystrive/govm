# Contributing to GoVM

Thank you for your interest in contributing to GoVM! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/wangystrive/govm.git
cd govm

# Build
cargo build --release

# Run tests
cargo test
```

### Project Structure

```
govm/
├── src/
�?  ├── main.rs       # Entry point
�?  ├── cli.rs        # CLI argument definitions
�?  ├── version.rs    # Go version parsing
�?  ├── download.rs   # Download functionality
�?  ├── install.rs    # Install/uninstall
�?  ├── switch.rs     # Version switching
�?  ├── system_go.rs  # System Go detection
�?  └── utils.rs      # Utilities
├── scripts/
�?  ├── install.sh    # Unix install script
�?  └── install.ps1   # Windows install script
├── .github/workflows/
�?  └── release.yml   # Release automation
├── Makefile          # Build automation
└── Cross.toml        # Cross-compilation config
```

## Making Changes

### Coding Standards

- Follow Rust naming conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Add tests for new functionality
- Update documentation as needed

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Building for Multiple Platforms

```bash
# Setup cross-compilation targets
make setup

# Build for all platforms
make release-all

# Or use cross directly
cross build --release --target x86_64-unknown-linux-musl
```

## Submitting Changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and ensure they pass
5. Update documentation if needed
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

### Code Review

- All submissions require review
- Address review comments promptly
- Be respectful and constructive

## Reporting Bugs

### Before Submitting

- Check if the bug has already been reported
- Try to reproduce with the latest version
- Gather relevant information

### Bug Report Template

```
**Description:**
Clear description of the bug

**Steps to Reproduce:**
1. Step one
2. Step two
3. ...

**Expected Behavior:**
What you expected to happen

**Actual Behavior:**
What actually happened

**Environment:**
- OS: [e.g., Windows 10, macOS 12, Ubuntu 22.04]
- GoVM Version: [e.g., 0.1.0]
- Shell: [e.g., PowerShell, Bash, Zsh]

**Additional Context:**
Any other relevant information
```

## Requesting Features

### Feature Request Template

```
**Description:**
Clear description of the feature

**Use Case:**
Why would this feature be useful?

**Proposed Solution:**
How do you think it should work?

**Alternatives:**
Any alternative solutions you've considered
```

## Release Process

1. Update `CHANGELOG.md`
2. Update version in `Cargo.toml`
3. Create a git tag: `git tag v0.x.x`
4. Push the tag: `git push origin v0.x.x`
5. GitHub Actions will build and create a release

## Community

- Be welcoming to newcomers
- Be respectful of differing viewpoints
- Accept constructive criticism
- Focus on what's best for the community

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
