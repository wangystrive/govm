# Release Checklist

Use this checklist when creating a new release of GoVM.

## Pre-Release

- [ ] Update `CHANGELOG.md` with new version and changes
- [ ] Update version in `Cargo.toml`
- [ ] Run tests: `cargo test`
- [ ] Check formatting: `cargo fmt --check`
- [ ] Run clippy: `cargo clippy -- -D warnings`
- [ ] Update README.md if needed
- [ ] Test install scripts
- [ ] Commit changes: `git commit -am "Prepare for vX.Y.Z"`

## Creating Release

### Option 1: Git Tag (Automated)

1. [ ] Create and push tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. [ ] GitHub Actions will automatically:
   - Build for all platforms
   - Create release with artifacts
   - Generate checksums

3. [ ] Verify the release on GitHub
   - Check all artifacts are present
   - Verify checksums
   - Test install scripts

### Option 2: Manual Release

1. [ ] Build for all platforms:
   ```bash
   make release-all
   ```

2. [ ] Create GitHub Release:
   - Go to GitHub Releases
   - Click "Draft a new release"
   - Select or create tag
   - Add release notes from CHANGELOG
   - Upload artifacts from `dist/`

## Post-Release

- [ ] Verify installation scripts work:
  ```bash
  # Unix
  curl -fsSL https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.sh | bash
  
  # Windows
  Invoke-WebRequest -Uri "https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.ps1" -OutFile "install.ps1"; .\install.ps1
  ```
- [ ] Close milestone if using GitHub milestones
- [ ] Announce release (if applicable)

## Platform Support Matrix

Ensure all platforms build successfully:

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 (gnu) | â¬?|
| Linux | x86_64 (musl) | â¬?|
| Linux | ARM64 | â¬?|
| Linux | ARMv7 | â¬?|
| macOS | x86_64 | â¬?|
| macOS | ARM64 (Apple Silicon) | â¬?|
| Windows | x86_64 | â¬?|
| Windows | x86 | â¬?|

## Artifact Checklist

- [ ] `govm-x86_64-unknown-linux-gnu.tar.gz`
- [ ] `govm-x86_64-unknown-linux-musl.tar.gz`
- [ ] `govm-aarch64-unknown-linux-gnu.tar.gz`
- [ ] `govm-armv7-unknown-linux-gnueabihf.tar.gz`
- [ ] `govm-x86_64-apple-darwin.tar.gz`
- [ ] `govm-aarch64-apple-darwin.tar.gz`
- [ ] `govm-x86_64-pc-windows-msvc.zip`
- [ ] `govm-i686-pc-windows-msvc.zip`
- [ ] `checksums.txt`

## Notes

- Version format: `v{major}.{minor}.{patch}` (Semantic Versioning)
- Always test install scripts after release
- Keep CHANGELOG up to date
